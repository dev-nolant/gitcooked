use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
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

use models::RecipeStore;
use rate_limiter::RateLimiter;
use app_state::AppState;



async fn serve_index() -> impl IntoResponse {
    let html = if cfg!(debug_assertions) {
        std::fs::read_to_string("templates/index.html").unwrap()
    } else {
        std::fs::read_to_string(format!("{}/index.min.html", env!("OUT_DIR"))).unwrap()
    };
    Html(html)
}

async fn serve_404() -> impl IntoResponse {
    let html = if cfg!(debug_assertions) {
        std::fs::read_to_string("templates/404.html").unwrap()
    } else {
        std::fs::read_to_string(format!("{}/404.min.html", env!("OUT_DIR"))).unwrap()
    };
    (StatusCode::NOT_FOUND, Html(html))
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
