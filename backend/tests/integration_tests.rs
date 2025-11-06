use std::convert::Infallible;

use anyhow::Result;
use app::response::ApiResponse;
use app::router::router;
use app::state::AppState;
use axum::body::to_bytes;
use axum::Router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use serial_test::serial;
use tower::ServiceExt;

async fn create_app() -> anyhow::Result<Router> {
    // Initialize the database and obtain a connection pool
    let state = AppState::new().await?;
    let app = router(state);
    Ok(app)
}

async fn parse_response<T: DeserializeOwned>(
    response: axum::response::Response,
) -> Result<ApiResponse<T>, Infallible> {
    // Extract the body as bytes
    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();

    // Deserialize the response body to ApiResponse for further assertions
    let api_response: ApiResponse<T> = serde_json::from_str(&body_text).unwrap();
    Ok(api_response)
}

#[cfg(test)]
mod test_integration {
    use super::*;
    use ctor::ctor;

    #[ctor]
    fn load_env() {
        let _ = dotenvy::dotenv();
        println!("✅ .env loaded automatically for tests");
    }

    #[tokio::test]
    #[serial]
    async fn test_health_status() -> Result<()> {
        // Test
        let request = Request::builder()
            .method("GET")
            .uri("/health")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        let app = create_app().await?;
        let response = app.oneshot(request).await.unwrap();

        let api_response: ApiResponse<String> = parse_response(response).await.unwrap();

        // Validate
        assert_eq!(api_response.code, StatusCode::OK);

        Ok(())
    }
}
