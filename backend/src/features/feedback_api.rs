use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use super::{feedback::FeedbackSubmission, feedback_store::FeedbackStore};

pub fn routes(store: FeedbackStore) -> Router {
    Router::new()
        .route("/feedback", post(submit))
        .with_state(store)
}

async fn submit(
    State(store): State<FeedbackStore>,
    Json(submission): Json<FeedbackSubmission>,
) -> Result<(StatusCode, Json<FeedbackReceipt>), ApiError> {
    let feedback = submission.validate().map_err(ApiError::validation)?;
    let record = store.submit(feedback);
    Ok((StatusCode::CREATED, Json(FeedbackReceipt { id: record.id })))
}

#[derive(Serialize)]
struct FeedbackReceipt {
    id: Uuid,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: &'static str,
}

struct ApiError {
    status: StatusCode,
    message: &'static str,
}

impl ApiError {
    fn validation(error: super::feedback::ValidationError) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: error.message(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: "validation_failed",
                message: self.message,
            }),
        )
            .into_response()
    }
}
