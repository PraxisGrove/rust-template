#![cfg_attr(test, allow(clippy::panic))]

use std::future::Future;
use std::pin::Pin;

use thiserror::Error;
use {{crate_name}}_domain::{HealthReport, HealthStatus};

/// Checks whether external dependencies required by the service are reachable.
///
/// Implementations should perform a bounded, side-effect-free readiness check
/// against infrastructure such as a database, queue, or cache.
pub trait ReadinessProbe: Send + Sync {
    fn check<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<ReadinessOutcome, ReadinessError>> + Send + 'a>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessOutcome {
    detail: String,
}

impl ReadinessOutcome {
    pub fn ready(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Error)]
#[error("readiness check failed: {reason}")]
pub struct ReadinessError {
    reason: String,
}

impl ReadinessError {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone)]
pub struct HealthService {
    service_name: String,
}

impl HealthService {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    pub fn liveness(&self) -> HealthReport {
        HealthReport::new(&self.service_name, HealthStatus::Live, "process is running")
    }

    pub async fn readiness(&self, probe: &dyn ReadinessProbe) -> HealthReport {
        match probe.check().await {
            Ok(outcome) => HealthReport::new(
                &self.service_name,
                HealthStatus::Ready,
                outcome.detail().to_owned(),
            ),
            Err(err) => HealthReport::new(
                &self.service_name,
                HealthStatus::NotReady,
                err.reason().to_owned(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PassingProbe;

    impl ReadinessProbe for PassingProbe {
        fn check<'a>(
            &'a self,
        ) -> Pin<Box<dyn Future<Output = Result<ReadinessOutcome, ReadinessError>> + Send + 'a>>
        {
            Box::pin(async { Ok(ReadinessOutcome::ready("dependencies are reachable")) })
        }
    }

    struct FailingProbe;

    impl ReadinessProbe for FailingProbe {
        fn check<'a>(
            &'a self,
        ) -> Pin<Box<dyn Future<Output = Result<ReadinessOutcome, ReadinessError>> + Send + 'a>>
        {
            Box::pin(async { Err(ReadinessError::new("database unavailable")) })
        }
    }

    #[test]
    fn reports_liveness() {
        let service = HealthService::new("service");

        let report = service.liveness();

        assert_eq!(HealthStatus::Live, report.status());
        assert_eq!("service", report.service());
    }

    #[tokio::test]
    async fn reports_ready_when_probe_passes() {
        let service = HealthService::new("service");

        let report = service.readiness(&PassingProbe).await;

        assert_eq!(HealthStatus::Ready, report.status());
        assert_eq!("dependencies are reachable", report.detail());
    }

    #[tokio::test]
    async fn reports_not_ready_when_probe_fails() {
        let service = HealthService::new("service");

        let report = service.readiness(&FailingProbe).await;

        assert_eq!(HealthStatus::NotReady, report.status());
        assert_eq!("database unavailable", report.detail());
    }
}
