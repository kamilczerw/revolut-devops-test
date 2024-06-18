use axum::{
    async_trait,
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};
use regex::Regex;

use crate::app::api::ApiError;

use super::api::UserBirthdayRequest;

#[async_trait]
impl<S> FromRequest<S> for UserBirthdayRequest
where
    Json<UserBirthdayRequest>: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Json::<UserBirthdayRequest>::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let Json(body) = body;
        validate_birthday_request(body).map_err(IntoResponse::into_response)
    }
}

fn validate_birthday_request(req: UserBirthdayRequest) -> Result<UserBirthdayRequest, ApiError> {
    // Create a regex to validate the date format.
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").map_err(|err| {
        log::error!("Failed to create regex: {}", err);
        ApiError::internal_server_error()
    })?;

    // Validate the date format.
    if !re.is_match(&req.date_of_birth) {
        return Err(ApiError::bad_request(
            "Invalid date format. Valid format: YYYY-MM-DD",
        ));
    }

    let date: chrono::NaiveDate = req.date_of_birth.parse().map_err(|err| {
        log::warn!("Failed to parse date: {}", err);
        ApiError::bad_request("Invalid date")
    })?;

    let date_earilest: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
    let today: chrono::NaiveDate = chrono::Local::now().date_naive();

    // Validate the date of birth.
    if date < date_earilest || date >= today {
        return Err(ApiError::bad_request(
            "Invalid date of birth. The date should be between 1900-01-01 and today.",
        ));
    }

    Ok(req)
}

#[cfg(test)]
mod tests {

    use axum::{
        body::{to_bytes, Body},
        http::header,
    };
    use chrono::Days;

    use super::*;

    fn request(body: &str) -> Request<Body> {
        Request::builder()
            .method("PUT")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    /// Map the response body to an `ApiError` struct.
    async fn get_response_error(response: Response<Body>) -> ApiError {
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body_bytes).unwrap()
    }

    #[tokio::test]
    async fn test_post_request_validation_with_invalid_dob_format() {
        let request = request(r#"{ "dateOfBirth": "foo" }"#);
        let result = UserBirthdayRequest::from_request(request, &()).await;
        assert!(result.is_err());

        if let Err(res) = result {
            let result = get_response_error(res).await;
            assert_eq!(result.status, 400);
            assert_eq!(
                result.message,
                "Invalid date format. Valid format: YYYY-MM-DD"
            );
        }
    }

    #[tokio::test]
    async fn test_post_request_validation_with_invalid_dob() {
        let request = request(r#"{ "dateOfBirth": "1899-12-31" }"#);
        let result = UserBirthdayRequest::from_request(request, &()).await;
        assert!(result.is_err());

        if let Err(res) = result {
            let res = get_response_error(res).await;
            assert_eq!(res.status, 400);
            assert_eq!(
                res.message,
                "Invalid date of birth. The date should be between 1900-01-01 and today."
            );
        }
    }

    #[tokio::test]
    async fn test_post_request_validation_with_dob_in_future() {
        let tomorrow: chrono::NaiveDate = chrono::Local::now()
            .date_naive()
            .checked_add_days(Days::new(1))
            .unwrap();

        let body = format!(r#"{{ "dateOfBirth": "{}" }}"#, tomorrow);
        let request = request(&body);

        let result = UserBirthdayRequest::from_request(request, &()).await;
        assert!(result.is_err());

        if let Err(res) = result {
            let res = get_response_error(res).await;
            assert_eq!(res.status, 400);
            assert_eq!(
                res.message,
                "Invalid date of birth. The date should be between 1900-01-01 and today."
            );
        }
    }

    #[tokio::test]
    async fn test_post_request_valiation_with_valid_data() {
        let request = request(r#"{ "dateOfBirth": "2000-12-31" }"#);
        let result = UserBirthdayRequest::from_request(request, &()).await;
        assert!(result.is_ok());

        if let Ok(res) = result {
            assert_eq!(res.date_of_birth, "2000-12-31");
        }
    }
}
