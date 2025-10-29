//! Graph Execution Module - ericadamsai watermark
//! Handles directed acyclic graph (DAG) based task execution and orchestration

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};

/// Represents a node in the execution graph
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub dependencies: Vec<String>,
    pub outputs: Vec<String>,
    pub metadata: serde_json::Value,
}

/// Type of node in the graph
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Task,
    Decision,
    Loop,
    Aggregator,
}

/// Represents an edge in the execution graph
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

/// Directed acyclic graph for execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionGraph {
    pub id: String,
    pub nodes: HashMap<String, GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub execution_order: Vec<String>,
}

impl ExecutionGraph {
    /// Create a new execution graph
    pub fn new(id: String) -> Self {
        info!("[ericadamsai] Creating ExecutionGraph: {}", id);
        Self {
            id,
            nodes: HashMap::new(),
            edges: Vec::new(),
            execution_order: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) -> Result<(), String> {
        if self.nodes.contains_key(&node.id) {
            return Err(format!("Node {} already exists", node.id));
        }
        debug!("[ericadamsai] Adding node to graph: {}", node.id);
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, edge: GraphEdge) -> Result<(), String> {
        if !self.nodes.contains_key(&edge.from) {
            return Err(format!("Source node {} not found", edge.from));
        }
        if !self.nodes.contains_key(&edge.to) {
            return Err(format!("Destination node {} not found", edge.to));
        }
        debug!("[ericadamsai] Adding edge: {} -> {}", edge.from, edge.to);
        self.edges.push(edge);
        Ok(())
    }

    /// Topologically sort the graph and determine execution order
    pub fn topological_sort(&mut self) -> Result<(), String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize
        for (node_id, _) in &self.nodes {
            in_degree.insert(node_id.clone(), 0);
            adjacency.insert(node_id.clone(), Vec::new());
        }

        // Build adjacency list and calculate in-degrees
        for edge in &self.edges {
            adjacency.entry(edge.from.clone()).or_insert_with(Vec::new).push(edge.to.clone());
            *in_degree.get_mut(&edge.to).unwrap() += 1;
        }

        // Kahn's algorithm
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut sorted_order = Vec::new();

        while let Some(node_id) = queue.pop_front() {
            sorted_order.push(node_id.clone());

            for neighbor in adjacency.get(&node_id).unwrap_or(&Vec::new()) {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        if sorted_order.len() != self.nodes.len() {
            return Err("Cycle detected in graph".to_string());
        }

        self.execution_order = sorted_order;
        info!("[ericadamsai] Graph topologically sorted");
        Ok(())
    }

    /// Get execution order
    pub fn get_execution_order(&self) -> Vec<String> {
        self.execution_order.clone()
    }

    /// Validate graph integrity
    pub fn validate(&self) -> Result<(), String> {
        for edge in &self.edges {
            if !self.nodes.contains_key(&edge.from) {
                return Err(format!("Edge references non-existent source node: {}", edge.from));
            }
            if !self.nodes.contains_key(&edge.to) {
                return Err(format!("Edge references non-existent destination node: {}", edge.to));
            }
        }
        info!("[ericadamsai] Graph validation passed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = ExecutionGraph::new("test-graph".to_string());
        assert_eq!(graph.id, "test-graph");
        assert_eq!(graph.nodes.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = ExecutionGraph::new("test-graph".to_string());
        let node = GraphNode {
            id: "node-1".to_string(),
            name: "Test Node".to_string(),
            node_type: NodeType::Task,
            dependencies: vec![],
            outputs: vec![],
            metadata: serde_json::json!({}),
        };
        assert!(graph.add_node(node).is_ok());
    }
}
