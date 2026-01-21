use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod handlers;
mod models;
mod github;
mod rate_limiter;
mod app_state;
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use models::RecipeStore;
use rate_limiter::RateLimiter;
use app_state::AppState;

const LOGO_PNG: &[u8] = include_bytes!("../public/logo/logo.png");



async fn serve_index() -> impl IntoResponse {
    Html(INDEX_HTML)
}

async fn serve_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(NOT_FOUND_HTML))
}

async fn serve_logo() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .body(axum::body::Body::from(LOGO_PNG))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let store = RecipeStore::default();
    models::load_recipes_from_disk(&store).await;

    let rate_limiter = Arc::new(RateLimiter::new(5, 60)); // 5 requests per hour

    let state = AppState {
        store,
        rate_limiter,
    };

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/recipes", get(serve_index))
        .route("/recipe/:id", get(serve_index))
        .route("/add", get(serve_index))
        .route("/public/logo/logo.png", get(serve_logo))
        .route("/api/recipes", get(handlers::get_all_recipes))
        .route("/api/recipes/issue", post(handlers::create_recipe_issue))
        .route("/api/recipes/:id/markdown", get(handlers::get_recipe_markdown))
        .route("/api/recipes/:id", get(handlers::get_recipe))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/public", ServeDir::new("public"))
        .fallback(serve_404)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("gitcooked.com running on http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
