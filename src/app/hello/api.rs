use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::store::BirthdayStore;
use super::validation::ValidatedUsername;
use crate::app::api::{ApiError, ApiResult};
use crate::app::Store;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserBirthdayRequest {
    pub date_of_birth: String,
}

impl UserBirthdayRequest {
    pub fn dob(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::parse_from_str(&self.date_of_birth, "%Y-%m-%d").unwrap()
    }
}

pub(crate) struct UserBirthdayResponse();

impl IntoResponse for UserBirthdayResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NO_CONTENT, Body::empty()).into_response()
    }
}

#[derive(serde::Serialize)]
pub(crate) struct GetBirthdayResponse {
    message: String,
}

/// API handler for upserting the day of birth for the requested user.
/// If the user doesn't exist, the handler will create a new record in the database.
pub(crate) async fn upsert_user(
    State(store): State<Store>,
    ValidatedUsername(username): ValidatedUsername,
    req: UserBirthdayRequest,
) -> ApiResult<UserBirthdayResponse> {
    log::trace!(
        "Upserting user birthday. Username: {}, dob: {}",
        &username,
        &req.date_of_birth
    );
    store.upsert_birthday(username, req.dob()).await?;

    Ok(UserBirthdayResponse())
}

pub(crate) async fn get_birthday(
    State(store): State<Store>,
    ValidatedUsername(username): ValidatedUsername,
) -> ApiResult<Json<GetBirthdayResponse>> {
    let birthday = store.get_birthday(&username).await?;

    if let Some(birthday) = birthday {
        Ok(Json(GetBirthdayResponse {
            message: format!("user: {}, birthday: {:?}", &username, &birthday),
        }))
    } else {
        Err(ApiError::not_found(&format!(
            "User '{}' was not found",
            &username
        )))
    }
    // let username = req.param("username").unwrap();
    // let birthday = db.get_birthday(username).await;
    // Ok(birthday)
}
