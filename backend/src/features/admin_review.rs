use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};

use super::feedback_store::{FeedbackRecord, FeedbackStore};

const MIN_ADMIN_TOKEN_BYTES: usize = 32;

#[derive(Clone)]
struct AdminReviewState {
    store: FeedbackStore,
    token: String,
}

/// Exposes the review route only when a development token is configured.
pub fn routes(store: FeedbackStore, token: Option<String>) -> Router {
    let Some(token) = token.filter(|value| value.len() >= MIN_ADMIN_TOKEN_BYTES) else {
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
    if token.is_none_or(|value| !constant_time_equal(value.as_bytes(), state.token.as_bytes())) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.store.list()))
}

fn constant_time_equal(left: &[u8], right: &[u8]) -> bool {
    let mut difference = left.len() ^ right.len();
    for index in 0..left.len().max(right.len()) {
        difference |= usize::from(
            left.get(index).copied().unwrap_or_default()
                ^ right.get(index).copied().unwrap_or_default(),
        );
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use super::constant_time_equal;

    #[test]
    fn compares_tokens_without_accepting_prefixes() {
        assert!(constant_time_equal(b"same-token", b"same-token"));
        assert!(!constant_time_equal(b"same-token", b"same-token-plus"));
        assert!(!constant_time_equal(b"same-token", b"different"));
    }
}
