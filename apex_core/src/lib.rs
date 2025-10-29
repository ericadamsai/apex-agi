//! Apex AGI Core Library
//! Watermark: apex-agi by OpenAI Assistant 2025-10-29

pub mod graph;
pub mod node;
pub mod edge;
pub mod scgo;
pub mod diagnostics;

use std::sync::{Arc, RwLock};
use graph::Graph;

#[derive(Clone)]
pub struct ApexCore {
    pub graph: Arc<RwLock<Graph>>,    
    pub seed: u64,
}

impl ApexCore {
    pub fn new(seed: u64) -> Self {
        Self { graph: Arc::new(RwLock::new(Graph::new())), seed }
    }
    pub fn deterministic_seed(&self) -> u64 { self.seed }
}

pub fn validate_graph(core: &ApexCore) -> diagnostics::Validation {
    let g = core.graph.read().unwrap();
    diagnostics::validate(&g)
}

pub fn patch_graph<F: FnOnce(&mut Graph)>(core: &ApexCore, patcher: F) -> diagnostics::Validation {
    let mut g = core.graph.write().unwrap();
    patcher(&mut g);
    diagnostics::validate(&g)
}

pub fn inspect_graph(core: &ApexCore) -> Graph { core.graph.read().unwrap().clone() }
