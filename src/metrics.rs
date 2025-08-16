use prometheus::{Histogram, HistogramOpts, IntCounter, IntGauge, Registry};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Metrics {
    // Request metrics
    pub requests_total: IntCounter,
    pub requests_in_progress: IntGauge,
    pub request_duration: Histogram,

    // Response metrics
    pub responses_total: IntCounter,
    pub response_status_codes: IntCounter,

    // Error metrics
    pub errors_total: IntCounter,
    pub request_errors: IntCounter,
    pub tls_errors: IntCounter,
    pub timeout_errors: IntCounter,

    // Connection metrics
    pub active_connections: IntGauge,
    pub connection_errors: IntCounter,

    // Registry for all metrics
    pub registry: Arc<RwLock<Registry>>,
}

impl Metrics {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let registry = Arc::new(RwLock::new(Registry::new()));

        // Request metrics
        let requests_total = IntCounter::new(
            "mtls_proxy_requests_total",
            "Total number of requests processed",
        )?;

        let requests_in_progress = IntGauge::new(
            "mtls_proxy_requests_in_progress",
            "Number of requests currently being processed",
        )?;

        let request_duration = Histogram::with_opts(
            HistogramOpts::new(
                "mtls_proxy_request_duration_seconds",
                "Request duration in seconds",
            )
            .buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0]),
        )?;

        // Response metrics
        let responses_total = IntCounter::new(
            "mtls_proxy_responses_total",
            "Total number of responses sent",
        )?;

        let response_status_codes = IntCounter::new(
            "mtls_proxy_response_status_codes_total",
            "Total number of responses by status code",
        )?;

        // Error metrics
        let errors_total = IntCounter::new("mtls_proxy_errors_total", "Total number of errors")?;

        let request_errors = IntCounter::new(
            "mtls_proxy_request_errors_total",
            "Total number of request errors",
        )?;

        let tls_errors =
            IntCounter::new("mtls_proxy_tls_errors_total", "Total number of TLS errors")?;

        let timeout_errors = IntCounter::new(
            "mtls_proxy_timeout_errors_total",
            "Total number of timeout errors",
        )?;

        // Connection metrics
        let active_connections = IntGauge::new(
            "mtls_proxy_active_connections",
            "Number of active connections",
        )?;

        let connection_errors = IntCounter::new(
            "mtls_proxy_connection_errors_total",
            "Total number of connection errors",
        )?;

        // Register all metrics
        let reg = registry.write().await;
        reg.register(Box::new(requests_total.clone()))?;
        reg.register(Box::new(requests_in_progress.clone()))?;
        reg.register(Box::new(request_duration.clone()))?;
        reg.register(Box::new(responses_total.clone()))?;
        reg.register(Box::new(response_status_codes.clone()))?;
        reg.register(Box::new(errors_total.clone()))?;
        reg.register(Box::new(request_errors.clone()))?;
        reg.register(Box::new(tls_errors.clone()))?;
        reg.register(Box::new(timeout_errors.clone()))?;
        reg.register(Box::new(active_connections.clone()))?;
        reg.register(Box::new(connection_errors.clone()))?;

        Ok(Self {
            requests_total,
            requests_in_progress,
            request_duration,
            responses_total,
            response_status_codes,
            errors_total,
            request_errors,
            tls_errors,
            timeout_errors,
            active_connections,
            connection_errors,
            registry: registry.clone(),
        })
    }

    pub async fn record_request_start(&self) {
        self.requests_total.inc();
        self.requests_in_progress.inc();
    }

    pub async fn record_request_end(&self, duration: f64) {
        self.requests_in_progress.dec();
        self.request_duration.observe(duration);
    }

    pub async fn record_response(&self, _status_code: u16) {
        self.responses_total.inc();
        self.response_status_codes.inc();
    }

    pub async fn record_error(&self, error_type: &str) {
        self.errors_total.inc();
        match error_type {
            "request" => self.request_errors.inc(),
            "tls" => self.tls_errors.inc(),
            "timeout" => self.timeout_errors.inc(),
            "connection" => self.connection_errors.inc(),
            _ => self.request_errors.inc(),
        }
    }

    pub async fn record_connection_start(&self) {
        self.active_connections.inc();
    }

    pub async fn record_connection_end(&self) {
        self.active_connections.dec();
    }

    pub async fn get_metrics(&self) -> Result<String, anyhow::Error> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = Vec::new();
        encoder.encode(&self.registry.read().await.gather(), &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}
