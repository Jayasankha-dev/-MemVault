use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use crate::scanner::scan_process_for_pattern;

#[derive(Deserialize)]
pub struct ScanRequest {
    pub pid: i32,
    pub pattern: String,
    pub entropy: Option<f64>,
}

#[derive(Serialize)]
pub struct ScanResponse {
    pub tokens: Vec<String>,
    pub count: usize,
}

async fn api_scan_handler(Json(req): Json<ScanRequest>) -> Json<ScanResponse> {
    let threshold = req.entropy.unwrap_or(5.8);
    let tokens = scan_process_for_pattern(req.pid, req.pattern.as_bytes(), threshold)
        .unwrap_or_default();
    Json(ScanResponse {
        count: tokens.len(),
        tokens,
    })
}

pub async fn run_api_server() {
    let app = Router::new().route("/scan", post(api_scan_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("{} MemVault API Server running on http://0.0.0.0:8080", "[*]".bold().blue());
    axum::serve(listener, app).await.unwrap();
}