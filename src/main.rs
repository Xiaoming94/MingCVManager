use axum::{
    Json,
    Router, //
    response::IntoResponse,
    routing::get,
};

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple rest API response";

    let json_response = serde_json::json!({
        "status": "connected",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/healthcheck", get(health_check_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
