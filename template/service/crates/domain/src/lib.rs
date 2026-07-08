#![cfg_attr(test, allow(clippy::panic))]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Live,
    Ready,
    NotReady,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    service: String,
    status: HealthStatus,
    detail: String,
}

impl HealthReport {
    pub fn new(
        service: impl Into<String>,
        status: HealthStatus,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            service: service.into(),
            status,
            detail: detail.into(),
        }
    }

    pub fn service(&self) -> &str {
        &self.service
    }

    pub fn status(&self) -> HealthStatus {
        self.status
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_health_report_values() {
        let report = HealthReport::new("service", HealthStatus::Ready, "database reachable");

        assert_eq!("service", report.service());
        assert_eq!(HealthStatus::Ready, report.status());
        assert_eq!("database reachable", report.detail());
    }
}
