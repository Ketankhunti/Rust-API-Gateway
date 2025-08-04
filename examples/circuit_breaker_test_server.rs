
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,

};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::net::TcpListener;
use tracing::info;

// This atomic boolean will control the health of our service.
// Arc allows it to be shared safely across threads.
#[derive(Clone)]
struct AppState {
    is_healthy: Arc<AtomicBool>,
}

// The main endpoint that our gateway will proxy to.
// It returns 200 or 500 based on the state.
async fn main_handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.is_healthy.load(Ordering::SeqCst) {
        info!("Handler called: Responding with 200 OK");
        (StatusCode::OK, "Hello from the healthy service!")
    } else {
        info!("Handler called: Responding with 500 Internal Server Error");
        (StatusCode::INTERNAL_SERVER_ERROR, "Service is currently failing!")
    }
}

// Control endpoints to change the health status.
async fn set_healthy(State(state): State<AppState>) -> &'static str {
    state.is_healthy.store(true, Ordering::SeqCst);
    info!("Service status set to HEALTHY");
    "Service is now healthy"
}

async fn set_unhealthy(State(state): State<AppState>) -> &'static str {
    state.is_healthy.store(false, Ordering::SeqCst);
    info!("Service status set to UNHEALTHY");
    "Service is now unhealthy"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let state = AppState {
        // Start in a failing state by default.
        is_healthy: Arc::new(AtomicBool::new(false)),
    };

    let app = Router::new()
        .route("/service/api/resilient", get(main_handler))
        .route("/control/healthy", post(set_healthy))
        .route("/control/unhealthy", post(set_unhealthy))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8001").await.unwrap();
    info!("Controllable backend listening on http://127.0.0.1:8001");
    axum::serve(listener, app).await.unwrap();
}
