#![forbid(unsafe_code)]

mod features;

use axum::{extract::DefaultBodyLimit, Router};
use features::feedback_store::FeedbackStore;

pub const MAX_BODY_BYTES: usize = 16 * 1024;

pub fn app() -> Router {
    let feedback_store = FeedbackStore::default();
    Router::new()
        .nest(
            "/api",
            features::health::routes().merge(features::feedback_api::routes(feedback_store)),
        )
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        routing::post,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_json_liveness() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "application/json");
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            serde_json::json!({"status": "ok"})
        );
    }

    #[tokio::test]
    async fn unknown_routes_return_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/feedback")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn feedback_submission_returns_an_opaque_receipt() {
        let response = app()
            .oneshot(
                Request::post("/api/feedback")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"category":"teaching","message":"Helpful examples"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let receipt: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(receipt["id"].as_str().is_some());
    }

    #[tokio::test]
    async fn feedback_submission_rejects_empty_messages() {
        let response = app()
            .oneshot(
                Request::post("/api/feedback")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"category":"other","message":" "}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            serde_json::json!({"error":"validation_failed","message":"message must not be empty"})
        );
    }

    #[tokio::test]
    async fn buffered_request_bodies_are_limited() {
        // Test-only route: no feedback ingestion endpoint is exposed yet.
        let router = Router::new()
            .route("/", post(|_: String| async { StatusCode::NO_CONTENT }))
            .layer(DefaultBodyLimit::max(MAX_BODY_BYTES));
        for (size, expected) in [
            (MAX_BODY_BYTES, StatusCode::NO_CONTENT),
            (MAX_BODY_BYTES + 1, StatusCode::PAYLOAD_TOO_LARGE),
        ] {
            let response = router
                .clone()
                .oneshot(
                    Request::post("/")
                        .body(Body::from("a".repeat(size)))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), expected);
        }
    }
}
