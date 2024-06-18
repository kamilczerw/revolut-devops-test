use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct ApiError {
    pub status: u16,
    pub message: String,
}

impl ApiError {
    pub(crate) fn new(status: impl Into<u16>, message: &str) -> Self {
        ApiError {
            status: status.into(),
            message: message.to_owned(),
        }
    }

    pub(crate) fn bad_request(message: &str) -> Self {
        ApiError::new(StatusCode::BAD_REQUEST, message)
    }

    pub(crate) fn internal_server_error() -> ApiError {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let status = self.status;
        let res = Json(self).into_response();
        let body: axum::body::Body = res.into_body();

        axum::http::Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap()
    }
}

impl From<axum::http::Response<Body>> for ApiError {
    fn from(res: axum::http::Response<Body>) -> Self {
        log::info!("res: {:?}", res);
        let message: String = format!("{:?}", res.body());
        ApiError::new(res.status().as_u16(), &message)
    }
}
