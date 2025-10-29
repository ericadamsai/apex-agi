//! Telemetry Module - ericadamsai watermark
//! Metrics collection, tracing, and observability features

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, debug, span, Level};

/// Telemetry metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub counters: HashMap<String, u64>,
    pub histograms: HashMap<String, Vec<f64>>,
    pub gauges: HashMap<String, f64>,
}

/// Telemetry collector
pub struct TelemetryCollector {
    metrics: Arc<Mutex<TelemetryMetrics>>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        info!("[ericadamsai] Initializing TelemetryCollector");
        Self {
            metrics: Arc::new(Mutex::new(TelemetryMetrics {
                counters: HashMap::new(),
                histograms: HashMap::new(),
                gauges: HashMap::new(),
            })),
        }
    }

    pub fn increment_counter(&self, name: &str, value: u64) {
        debug!("[ericadamsai] Incrementing counter: {}", name);
        let mut metrics = self.metrics.lock().unwrap();
        *metrics.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn record_histogram(&self, name: &str, value: f64) {
        debug!("[ericadamsai] Recording histogram: {}", name);
        let mut metrics = self.metrics.lock().unwrap();
        metrics.histograms
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        debug!("[ericadamsai] Setting gauge: {}", name);
        let mut metrics = self.metrics.lock().unwrap();
        metrics.gauges.insert(name.to_string(), value);
    }

    pub fn get_metrics(&self) -> TelemetryMetrics {
        self.metrics.lock().unwrap().clone()
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_collector() {
        let collector = TelemetryCollector::new();
        collector.increment_counter("requests", 1);
        let metrics = collector.get_metrics();
        assert_eq!(metrics.counters.get("requests"), Some(&1));
    }
}
