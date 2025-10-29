//! Apex Server Main - ericadamsai watermark
//! RESTful API server for AGI system

use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::info;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
    pub id: String,
    pub description: String,
    pub priority: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: String,
    pub status: String,
    pub result: Option<String>,
}

async fn health_check() -> HttpResponse {
    info!("[ericadamsai] Health check endpoint called");
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy", "version": "0.1.0"}))
}

async fn create_task(req: web::Json<TaskRequest>) -> HttpResponse {
    info!("[ericadamsai] Creating task: {}", req.id);
    HttpResponse::Created().json(TaskResponse {
        id: req.id.clone(),
        status: "created".to_string(),
        result: None,
    })
}

async fn get_task(id: web::Path<String>) -> HttpResponse {
    info!("[ericadamsai] Fetching task: {}", id);
    HttpResponse::Ok().json(TaskResponse {
        id: id.to_string(),
        status: "pending".to_string(),
        result: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("[ericadamsai] Starting Apex Server");
    
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::get().to(get_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}
