use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    response::Response,
    Json,
};
use serde_json::json;

use crate::github::{format_recipe_issue, GitHubConfig};
use crate::models::CreateRecipeRequest;
use crate::app_state::AppState as AppStateAlias;

pub async fn get_all_recipes(State(state): State<AppStateAlias>) -> impl IntoResponse {
    let recipes = state.store.read().await;
    Json(recipes.clone())
}

pub async fn get_recipe(
    State(state): State<AppStateAlias>,
    Path(id): Path<String>,
) -> Response {
    let recipes = state.store.read().await;
    
    match recipes.iter().find(|r| r.id == id) {
        Some(recipe) => (StatusCode::OK, Json(recipe.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({ "error": "Recipe not found" }))).into_response(),
    }
}

pub async fn create_recipe_issue(
    State(state): State<AppStateAlias>,
    Json(req): Json<CreateRecipeRequest>,
) -> impl IntoResponse {
    let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    match state.rate_limiter.check_rate_limit(ip).await {
        Ok(()) => {},
        Err(e) => {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": e.to_string(),
                    "retry_after": e.retry_after().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
                }))
            ).into_response();
        }
    }

    let combined_content = format!("{} {} {}", req.title, req.description, req.ingredients.join(" ")).to_lowercase();

    let spam_keywords = ["buy now", "click here", "free money", "viagra", "casino", "lottery", "test submission"];
    for keyword in spam_keywords {
        if combined_content.contains(keyword) {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Spam detected. Submission rejected." }))
            ).into_response();
        }
    }

    let config = GitHubConfig::default();
    let title = &format!("Recipe: {}", req.title);
    let body = format_recipe_issue(&req);
    let issue_url = config.issue_url(title, &body);

    (StatusCode::OK, Json(json!({ "issue_url": issue_url }))).into_response()
}

pub async fn get_recipe_markdown(
    State(state): State<AppStateAlias>,
    Path(id): Path<String>,
) -> Response {
    let recipes = state.store.read().await;
    
    match recipes.iter().find(|r| r.id == id) {
        Some(recipe) => (
            [(header::CONTENT_TYPE, "text/markdown")],
            recipe.to_markdown(),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Recipe not found".to_string(),
        ).into_response(),
    }
}
