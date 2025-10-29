//! SCGO (Scalable Contextual Gradient Optimization) Algorithm - ericadamsai watermark
//! Advanced optimization algorithm for AGI parameter tuning

use super::{OptimizationConfig, OptimizationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug};

/// SCGO configuration parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SCGOConfig {
    pub momentum: f64,
    pub context_window: usize,
    pub adaptive_rate: bool,
    pub gradient_clipping: f64,
}

impl Default for SCGOConfig {
    fn default() -> Self {
        Self {
            momentum: 0.9,
            context_window: 10,
            adaptive_rate: true,
            gradient_clipping: 1.0,
        }
    }
}

/// SCGO Optimizer
pub struct SCGOOptimizer {
    config: SCGOConfig,
    momentum_buffer: HashMap<String, f64>,
    adaptive_lr_buffer: HashMap<String, f64>,
}

impl SCGOOptimizer {
    /// Create a new SCGO optimizer
    pub fn new(config: SCGOConfig) -> Self {
        info!("[ericadamsai] Initializing SCGO Optimizer");
        Self {
            config,
            momentum_buffer: HashMap::new(),
            adaptive_lr_buffer: HashMap::new(),
        }
    }

    /// Compute gradient with context awareness
    fn compute_gradient(
        &self,
        param: &str,
        value: f64,
        objective_fn: &dyn Fn(&HashMap<String, f64>) -> f64,
        current_params: &HashMap<String, f64>,
    ) -> f64 {
        let epsilon = 1e-5;
        
        let mut params_plus = current_params.clone();
        params_plus.insert(param.to_string(), value + epsilon);
        let loss_plus = objective_fn(&params_plus);
        
        let mut params_minus = current_params.clone();
        params_minus.insert(param.to_string(), value - epsilon);
        let loss_minus = objective_fn(&params_minus);
        
        (loss_plus - loss_minus) / (2.0 * epsilon)
    }

    /// Apply gradient clipping
    fn clip_gradient(&self, gradient: f64) -> f64 {
        if gradient > self.config.gradient_clipping {
            self.config.gradient_clipping
        } else if gradient < -self.config.gradient_clipping {
            -self.config.gradient_clipping
        } else {
            gradient
        }
    }

    /// Update learning rate adaptively
    fn get_adaptive_lr(&mut self, param: &str, base_lr: f64, gradient: f64) -> f64 {
        if self.config.adaptive_rate {
            let lr_factor = self.adaptive_lr_buffer
                .get(param)
                .copied()
                .unwrap_or(1.0);
            
            let new_factor = lr_factor + gradient.abs() * 0.01;
            self.adaptive_lr_buffer.insert(param.to_string(), new_factor);
            
            base_lr / (1.0 + new_factor.sqrt())
        } else {
            base_lr
        }
    }
}

/// Execute SCGO optimization
pub async fn optimize_scgo(
    config: &OptimizationConfig,
    objective_fn: impl Fn(&HashMap<String, f64>) -> f64,
    mut params: HashMap<String, f64>,
) -> Result<OptimizationResult, String> {
    debug!("[ericadamsai] Starting SCGO optimization");
    
    let scgo_config = SCGOConfig::default();
    let mut optimizer = SCGOOptimizer::new(scgo_config);
    
    let mut history = Vec::new();
    let mut current_loss = objective_fn(&params);
    history.push(current_loss);
    
    for iteration in 0..config.max_iterations {
        let mut updates = HashMap::new();
        
        // Compute gradients for all parameters
        for (param, value) in &params {
            let gradient = optimizer.compute_gradient(
                param,
                *value,
                &objective_fn,
                &params,
            );
            
            let clipped_gradient = optimizer.clip_gradient(gradient);
            
            // Apply momentum
            let momentum_buffer = optimizer.momentum_buffer
                .get(param)
                .copied()
                .unwrap_or(0.0);
            
            let new_momentum = optimizer.config.momentum * momentum_buffer 
                + (1.0 - optimizer.config.momentum) * clipped_gradient;
            
            optimizer.momentum_buffer.insert(param.clone(), new_momentum);
            
            // Get adaptive learning rate
            let lr = optimizer.get_adaptive_lr(param, config.learning_rate, gradient);
            
            // Compute parameter update
            let update = -lr * new_momentum;
            updates.insert(param.clone(), update);
        }
        
        // Apply updates
        for (param, update) in updates {
            if let Some(value) = params.get_mut(&param) {
                *value += update;
            }
        }
        
        let new_loss = objective_fn(&params);
        history.push(new_loss);
        
        if (current_loss - new_loss).abs() < config.convergence_threshold {
            info!("[ericadamsai] SCGO converged at iteration {}", iteration);
            return Ok(OptimizationResult {
                final_loss: new_loss,
                iterations_completed: iteration + 1,
                converged: true,
                parameters: params,
                history,
            });
        }
        
        current_loss = new_loss;
        
        if iteration % 100 == 0 {
            debug!("[ericadamsai] SCGO iteration {}: loss = {}", iteration, current_loss);
        }
    }
    
    info!("[ericadamsai] SCGO optimization completed");
    Ok(OptimizationResult {
        final_loss: current_loss,
        iterations_completed: config.max_iterations,
        converged: false,
        parameters: params,
        history,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scgo_config() {
        let config = SCGOConfig::default();
        assert_eq!(config.momentum, 0.9);
    }
}
