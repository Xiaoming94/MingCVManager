use axum::{
    Json,
    Router, //
    response::IntoResponse,
    routing::get,
};

mod users;

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "alive and ready";

    let json_response = serde_json::json!({
        "status": "connected",
        "message": MESSAGE
    });

    Json(json_response)
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1.0/healthcheck", get(health_check_handler))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = create_app();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(feature = "integration")]
mod integration_tests;
