use std::sync::Arc;

use crate::models::RecipeStore;
use crate::rate_limiter::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub store: RecipeStore,
    pub rate_limiter: Arc<RateLimiter>,
}
