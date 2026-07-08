#![cfg_attr(test, allow(clippy::panic, clippy::panic_in_result_fn))]

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use {{crate_name}}_app::{HealthService, ReadinessProbe};
use {{crate_name}}_domain::{HealthReport, HealthStatus};
use {{crate_name}}_infra::PostgresReadinessProbe;

const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:3000";
const SERVICE_NAME: &str = "{{project-name}}";

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let config = ServiceConfig::from_env()?;
    let readiness_probe = PostgresReadinessProbe::connect(&config.database_url)
        .await
        .context("failed to connect to PostgreSQL")?;
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
    database_url: String,
}

impl ServiceConfig {
    fn from_env() -> Result<Self> {
        let bind_address = env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_owned())
            .parse()
            .context("BIND_ADDRESS must be a socket address")?;
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;

        Ok(Self {
            bind_address,
            database_url,
        })
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

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .json()
        .init();
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
        fn check<'a>(
            &'a self,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = std::result::Result<(), {{crate_name}}_app::ReadinessError>,
                    > + Send
                    + 'a,
            >,
        > {
            Box::pin(async { Ok(()) })
        }
    }

    struct FailingProbe;

    impl ReadinessProbe for FailingProbe {
        fn check<'a>(
            &'a self,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = std::result::Result<(), {{crate_name}}_app::ReadinessError>,
                    > + Send
                    + 'a,
            >,
        > {
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
}
