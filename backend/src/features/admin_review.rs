use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};

use super::feedback_store::{FeedbackRecord, FeedbackStore};

#[derive(Clone)]
struct AdminReviewState {
    store: FeedbackStore,
    token: String,
}

/// Exposes the review route only when a development token is configured.
pub fn routes(store: FeedbackStore, token: Option<String>) -> Router {
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        return Router::new();
    };
    Router::new()
        .route("/admin/feedback", get(list_feedback))
        .with_state(AdminReviewState { store, token })
}

async fn list_feedback(
    State(state): State<AdminReviewState>,
    headers: HeaderMap,
) -> Result<Json<Vec<FeedbackRecord>>, StatusCode> {
    let token = headers
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok());
    if token != Some(state.token.as_str()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.store.list()))
}
