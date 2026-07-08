#![cfg_attr(test, allow(clippy::panic, clippy::panic_in_result_fn))]

use std::env;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use {{crate_name}}_app::{HealthService, ReadinessOutcome, ReadinessProbe};
use {{crate_name}}_domain::{HealthReport, HealthStatus};
use {{crate_name}}_infra::PostgresReadinessProbe;

const DEFAULT_APP_ADDR: &str = "127.0.0.1:3000";
const SERVICE_NAME: &str = "{{project-name}}";
type ReadinessFuture<'a> = Pin<
    Box<
        dyn Future<
                Output = std::result::Result<ReadinessOutcome, {{crate_name}}_app::ReadinessError>,
            > + Send
            + 'a,
    >,
>;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ServiceConfig::from_env()?;
    let _telemetry = init_tracing(&config)?;
    let readiness_probe = readiness_probe(&config).await?;
    let state = AppState::new(HealthService::new(SERVICE_NAME), Arc::new(readiness_probe));
    let listener = TcpListener::bind(config.bind_address)
        .await
        .with_context(|| format!("failed to bind {}", config.bind_address))?;

    info!(addr = %config.bind_address, "service listening");
    axum::serve(listener, router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server failed")?;

    Ok(())
}

#[derive(Debug, Clone)]
struct ServiceConfig {
    bind_address: SocketAddr,
    database_url: Option<String>,
    otel_enabled: bool,
    otel_service_name: String,
}

impl ServiceConfig {
    fn from_env() -> Result<Self> {
        let bind_address = env::var("APP_ADDR")
            .unwrap_or_else(|_| DEFAULT_APP_ADDR.to_owned())
            .parse()
            .context("APP_ADDR must be a socket address")?;
        let database_url = non_empty_env("DATABASE_URL");
        let otel_enabled = non_empty_env("OTEL_EXPORTER_OTLP_ENDPOINT").is_some();
        let otel_service_name =
            non_empty_env("OTEL_SERVICE_NAME").unwrap_or_else(|| SERVICE_NAME.to_owned());

        Ok(Self {
            bind_address,
            database_url,
            otel_enabled,
            otel_service_name,
        })
    }
}

fn non_empty_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

async fn readiness_probe(config: &ServiceConfig) -> Result<ReadinessProbeKind> {
    match &config.database_url {
        Some(database_url) => PostgresReadinessProbe::connect(database_url)
            .await
            .context("failed to connect to PostgreSQL")
            .map(ReadinessProbeKind::Postgres),
        None => Ok(ReadinessProbeKind::Skipped(SkippedReadinessProbe)),
    }
}

#[derive(Debug, Clone)]
enum ReadinessProbeKind {
    Postgres(PostgresReadinessProbe),
    Skipped(SkippedReadinessProbe),
}

impl ReadinessProbe for ReadinessProbeKind {
    fn check<'a>(&'a self) -> ReadinessFuture<'a> {
        match self {
            Self::Postgres(probe) => probe.check(),
            Self::Skipped(probe) => probe.check(),
        }
    }
}

#[derive(Debug, Clone)]
struct SkippedReadinessProbe;

impl ReadinessProbe for SkippedReadinessProbe {
    fn check<'a>(&'a self) -> ReadinessFuture<'a> {
        Box::pin(async { Ok(ReadinessOutcome::ready("database readiness check skipped")) })
    }
}

#[derive(Clone)]
struct AppState {
    health: HealthService,
    readiness_probe: Arc<dyn ReadinessProbe>,
}

impl AppState {
    fn new(health: HealthService, readiness_probe: Arc<dyn ReadinessProbe>) -> Self {
        Self {
            health,
            readiness_probe,
        }
    }
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .with_state(state)
}

async fn liveness(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse::from_report(state.health.liveness()))
}

async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let report = state.health.readiness(state.readiness_probe.as_ref()).await;
    let status = match report.status() {
        HealthStatus::Live | HealthStatus::Ready => StatusCode::OK,
        HealthStatus::NotReady => StatusCode::SERVICE_UNAVAILABLE,
    };

    (status, Json(HealthResponse::from_report(report)))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    service: String,
    status: &'static str,
    detail: String,
}

impl HealthResponse {
    fn from_report(report: HealthReport) -> Self {
        let status = match report.status() {
            HealthStatus::Live => "live",
            HealthStatus::Ready => "ready",
            HealthStatus::NotReady => "not_ready",
        };

        Self {
            service: report.service().to_owned(),
            status,
            detail: report.detail().to_owned(),
        }
    }
}

fn init_tracing(config: &ServiceConfig) -> Result<TelemetryGuard> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer().json();

    if !config.otel_enabled {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
        return Ok(TelemetryGuard::Disabled);
    }

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .context("failed to build OTLP trace exporter")?;
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name(config.otel_service_name.clone())
                .build(),
        )
        .build();
    let tracer = provider.tracer(config.otel_service_name.clone());
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(otel_layer)
        .init();

    Ok(TelemetryGuard::Enabled(provider))
}

#[derive(Debug)]
enum TelemetryGuard {
    Enabled(SdkTracerProvider),
    Disabled,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Self::Enabled(provider) = self
            && let Err(err) = provider.shutdown()
        {
            tracing::warn!(error = ?err, "failed to shut down OpenTelemetry");
        }
    }
}

async fn shutdown_signal() {
    if let Err(err) = tokio::signal::ctrl_c().await {
        tracing::warn!(error = %err, "failed to listen for shutdown signal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use tower::ServiceExt;

    struct PassingProbe;

    impl ReadinessProbe for PassingProbe {
        fn check<'a>(&'a self) -> ReadinessFuture<'a> {
            Box::pin(async { Ok(ReadinessOutcome::ready("dependencies are reachable")) })
        }
    }

    struct FailingProbe;

    impl ReadinessProbe for FailingProbe {
        fn check<'a>(&'a self) -> ReadinessFuture<'a> {
            Box::pin(async {
                Err({{crate_name}}_app::ReadinessError::new(
                    "database unavailable",
                ))
            })
        }
    }

    #[tokio::test]
    async fn liveness_returns_ok() -> Result<()> {
        let app = router(AppState::new(
            HealthService::new(SERVICE_NAME),
            Arc::new(PassingProbe),
        ));

        let response = app
            .oneshot(Request::get("/health/live").body(Body::empty())?)
            .await?;

        assert_eq!(StatusCode::OK, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn readiness_returns_unavailable_when_probe_fails() -> Result<()> {
        let app = router(AppState::new(
            HealthService::new(SERVICE_NAME),
            Arc::new(FailingProbe),
        ));

        let response = app
            .oneshot(Request::get("/health/ready").body(Body::empty())?)
            .await?;
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1024).await?;
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(StatusCode::SERVICE_UNAVAILABLE, status);
        assert_eq!("not_ready", payload["status"]);
        Ok(())
    }

    #[tokio::test]
    async fn readiness_returns_ok_when_database_is_not_configured() -> Result<()> {
        let app = router(AppState::new(
            HealthService::new(SERVICE_NAME),
            Arc::new(SkippedReadinessProbe),
        ));

        let response = app
            .oneshot(Request::get("/health/ready").body(Body::empty())?)
            .await?;
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1024).await?;
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(StatusCode::OK, status);
        assert_eq!("ready", payload["status"]);
        assert_eq!("database readiness check skipped", payload["detail"]);
        Ok(())
    }
}
