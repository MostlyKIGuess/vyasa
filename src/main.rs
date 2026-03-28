mod indicators;
mod models;
mod scoring;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use models::{DetectRequest, DetectorConfig};
use scoring::create_detection_result;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    config: Arc<DetectorConfig>,
}

#[tokio::main]
async fn main() {
    let config = Arc::new(DetectorConfig::default());
    let state = AppState { config };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/detect", post(detect))
        .route("/api/health", get(health))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0], 6767));
    println!("Vyasa listening on http://{}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind to port 6767");

    axum::serve(listener, app)
        .await
        .expect("server crashed unexpectedly");
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "vyasa",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn detect(
    State(state): State<AppState>,
    Json(request): Json<DetectRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let text = request.text.trim();
    if text.is_empty() {
        return Ok(Json(serde_json::json!({
            "error": "Text cannot be empty"
        })));
    }

    // Hard server cap at 5MB, client can request lower via max_chars
    let server_max: usize = 5_000_000;
    let max_chars = request.max_chars.unwrap_or(1_000_000).min(server_max);

    if text.len() > max_chars {
        return Ok(Json(serde_json::json!({
            "error": format!("Text too long ({} chars, max {})", text.len(), max_chars)
        })));
    }

    let (breakdown, flagged_phrases) = indicators::analyze_indicators(text);
    let result = create_detection_result(text, breakdown, flagged_phrases, &state.config);

    let json_value =
        serde_json::to_value(&result).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json_value))
}
