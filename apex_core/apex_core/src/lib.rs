//! Apex AGI Core Library - watermark: ericadamsai

pub mod engine;
pub mod graph;
pub mod opt;
pub mod persist;
pub mod telemetry;

pub use engine::{ApexEngine, EngineConfig};
pub use graph::{ExecutionNode, ExecutionEdge, Graph, GraphExecutor};
pub use opt::scgo::{ScgoConfig, ScgoOptimizer};

use tracing::{info, Level};

pub fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();
    info!(target = "apex_core", "tracing initialized (ericadamsai)");
}
