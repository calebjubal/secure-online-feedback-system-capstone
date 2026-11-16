#![forbid(unsafe_code)]

mod features;

use axum::{extract::DefaultBodyLimit, Router};

pub const MAX_BODY_BYTES: usize = 16 * 1024;

pub fn app() -> Router {
    Router::new()
        .nest("/api", features::health::routes())
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
