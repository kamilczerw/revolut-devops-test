use std::fmt::Debug;

use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Path, Request},
    response::{IntoResponse, Response},
    Json,
};
use regex::Regex;

use crate::app::api::ApiError;

pub struct ValidatedUsername(pub String);

/// Implement the `FromRequest` extractor for the `ValidatedUsername` struct.
/// This will allow Axum to automatically validate the username and extract it from the request path.
#[async_trait]
impl<S> FromRequestParts<S> for ValidatedUsername
where
    Path<String>: FromRequestParts<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        println!("Parts: {:?}", parts);
        let username = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let Path(username) = username;

        validate_username(&username)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(ValidatedUsername(username))
    }
}

/// The actual validation logic for the username.
async fn validate_username(username: &str) -> Result<(), ApiError> {
    let re = Regex::new(r"^[a-zA-Z]+$").map_err(|err| {
        log::error!("Failed to create regex: {}", err);
        ApiError::internal_server_error()
    })?;
    if username.is_empty() {
        return Err(ApiError::bad_request("Username should not be empty."));
    }

    if !re.is_match(username) {
        return Err(ApiError::bad_request(
            "Invalid username. Only letters are allowed.",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use axum::{
        body::{to_bytes, Body},
        http::StatusCode,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use super::*;

    fn request(username: &str) -> Request<Body> {
        Request::get(format!("/hello/{}", username))
            .body(Body::empty())
            .unwrap()
    }

    fn router() -> Router {
        Router::new().route(
            "/hello/:username",
            get(
                |ValidatedUsername(username): ValidatedUsername| async move {
                    assert!(!username.is_empty());
                },
            ),
        )
    }

    /// Map the response body to an `ApiError` struct.
    async fn get_response_error(response: Response<Body>) -> ApiError {
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body_bytes).unwrap()
    }

    #[tokio::test]
    async fn test_username_validation_with_valid_user() {
        let res = router().oneshot(request("foo")).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_username_validation_with_invalid_username() {
        let res = router().oneshot(request("foo-bar")).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let res = get_response_error(res).await;
        assert_eq!(res.status, 400);
        assert_eq!(res.message, "Invalid username. Only letters are allowed.");
    }
}
