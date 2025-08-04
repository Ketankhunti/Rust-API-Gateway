use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone)]
struct AppState {
    is_failing: Arc<AtomicBool>,
    request_counter: Arc<AtomicU64>,
}

async fn public_handler() -> &'static str { "public ok" }
async fn private_handler() -> &'static str { "private ok" }

// This handler includes a timestamp to prove the cache is working.
async fn cacheable_handler(State(state): State<AppState>) -> Json<Value> {
    let count = state.request_counter.fetch_add(1, Ordering::SeqCst);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    Json(json!({ "timestamp": now, "request_count": count }))
}

async fn failing_handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.is_failing.load(Ordering::SeqCst) {
        (StatusCode::INTERNAL_SERVER_ERROR, "service is failing")
    } else {
        (StatusCode::OK, "service is healthy")
    }
}

async fn set_healthy(State(state): State<AppState>) { state.is_failing.store(false, Ordering::SeqCst); }
async fn set_unhealthy(State(state): State<AppState>) { state.is_failing.store(true, Ordering::SeqCst); }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let state = AppState {
        is_failing: Arc::new(AtomicBool::new(true)), // Start in a failing state
        request_counter: Arc::new(AtomicU64::new(0)),
    };
    let app = Router::new()
        .route("/public", get(public_handler))
        .route("/private", get(private_handler))
        .route("/cacheable", get(cacheable_handler))
        .route("/failing", get(failing_handler))
        .route("/control/healthy", post(set_healthy))
        .route("/control/unhealthy", post(set_unhealthy))
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    info!("Comprehensive backend listening on http://127.0.0.1:8000");
    axum::serve(listener, app).await.unwrap();
}