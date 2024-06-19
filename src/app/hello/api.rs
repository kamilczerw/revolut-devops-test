use anyhow::anyhow;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Datelike, NaiveDate};

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
    pub fn dob(&self) -> anyhow::Result<chrono::NaiveDate> {
        let date = chrono::NaiveDate::parse_from_str(&self.date_of_birth, "%Y-%m-%d")?;
        Ok(date)
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

impl GetBirthdayResponse {
    /// Create a new `GetBirthdayResponse` instance.
    ///
    /// The response message will be different depending on the user's birthday.
    ///   - If the birthday is today, the message will be "Hello, {username}! Happy birthday!".
    ///   - Otherwise, the message will be "Hello, {username}! Your birthday is in {days_until_birthday} day(s)".
    pub fn new(username: &str, dob: &chrono::NaiveDate) -> anyhow::Result<Self> {
        let now: NaiveDate = chrono::Local::now().date_naive();

        let dob = Self::this_year(dob)?;
        let message = if dob == now {
            format!("Hello, {}! Happy birthday!", username)
        } else {
            let mut days_until_birthday = dob.signed_duration_since(now).num_days();
            if days_until_birthday < 0 {
                let birthday_next_year = Self::next_year(&dob)?;
                days_until_birthday = birthday_next_year.signed_duration_since(now).num_days();
            }
            format!(
                "Hello, {}! Your birthday is in {} day(s)",
                username, days_until_birthday
            )
        };

        Ok(GetBirthdayResponse { message })
    }

    fn this_year(date: &chrono::NaiveDate) -> anyhow::Result<chrono::NaiveDate> {
        let now: NaiveDate = chrono::Local::now().date_naive();
        if let Some(date) = date.with_year(now.year()) {
            Ok(date)
        } else {
            Err(anyhow!("Failed to set the year to the current year"))
        }
    }

    fn next_year(date: &chrono::NaiveDate) -> anyhow::Result<chrono::NaiveDate> {
        let now: NaiveDate = chrono::Local::now().date_naive();
        if let Some(date) = date.with_year(now.year() + 1) {
            Ok(date)
        } else {
            Err(anyhow!("Failed to set the year to the next year"))
        }
    }
}

/// API handler for upserting the day of birth for the requested user.
/// If the user doesn't exist, the handler will create a new record in the database.
pub(crate) async fn upsert_user(
    State(store): State<Store>,
    ValidatedUsername(username): ValidatedUsername,
    req: UserBirthdayRequest,
) -> ApiResult<UserBirthdayResponse> {
    log::debug!(
        "Upserting user birthday. Username: {}, dob: {}",
        &username,
        &req.date_of_birth
    );

    let dob = req.dob()?;
    store.upsert_birthday(username, dob).await?;

    Ok(UserBirthdayResponse())
}

/// API handler for getting the birthday for the requested user.
/// If the user doesn't exist, the handler will return a 404.
pub(crate) async fn get_birthday(
    State(store): State<Store>,
    ValidatedUsername(username): ValidatedUsername,
) -> ApiResult<Json<GetBirthdayResponse>> {
    log::debug!("Getting birthday for user: {}", &username);
    let birthday = store.get_birthday(&username).await?;

    if let Some(birthday) = birthday {
        let response = GetBirthdayResponse::new(&username, &birthday.dob)?;
        Ok(Json(response))
    } else {
        Err(ApiError::not_found(&format!(
            "User '{}' was not found",
            &username
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_user_birthday_request_dob() {
        let req = UserBirthdayRequest {
            date_of_birth: "2021-01-01".to_string(),
        };

        let res = req.dob();
        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
        }
    }

    #[tokio::test]
    async fn test_upsert_user_birthday() {
        let store = Store::new_in_mem().await.unwrap();

        let res = upsert_user(
            State(store),
            ValidatedUsername("foo".to_owned()),
            UserBirthdayRequest {
                date_of_birth: "2021-01-01".to_owned(),
            },
        )
        .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_birthday() {
        let this_year = chrono::Local::now().naive_local().date().year();
        let tomorrow_last_year = chrono::Local::now()
            .naive_local()
            .date()
            .succ_opt()
            .unwrap()
            .with_year(this_year - 1)
            .unwrap();

        let store = Store::new_in_mem().await.unwrap();
        store
            .upsert_birthday("foo".to_owned(), tomorrow_last_year)
            .await
            .unwrap();

        let res = get_birthday(State(store), ValidatedUsername("foo".to_owned())).await;

        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.message, "Hello, foo! Your birthday is in 1 day(s)");
        }
    }

    #[tokio::test]
    async fn test_get_birthday_response_with_one_day_until_birthday() {
        let this_year = chrono::Local::now().naive_local().date().year();
        let tomorrow_last_year = chrono::Local::now()
            .naive_local()
            .date()
            .succ_opt()
            .unwrap()
            .with_year(this_year - 1)
            .unwrap();

        let res = GetBirthdayResponse::new("foo", &tomorrow_last_year);

        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.message, "Hello, foo! Your birthday is in 1 day(s)");
        }
    }

    #[tokio::test]
    async fn test_get_birthday_response_with_birthday_today() {
        let this_year = chrono::Local::now().naive_local().date().year();
        let today_last_year = chrono::Local::now()
            .naive_local()
            .date()
            .with_year(this_year - 1)
            .unwrap();

        let res = GetBirthdayResponse::new("foo", &today_last_year);

        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.message, "Hello, foo! Happy birthday!");
        }
    }

    #[tokio::test]
    async fn test_get_birthday_response_with_birthday_already_passed() {
        let this_year = chrono::Local::now().naive_local().date().year();
        let yesterday_last_year = chrono::Local::now()
            .naive_local()
            .date()
            .pred_opt()
            .unwrap()
            .with_year(this_year - 1)
            .unwrap();

        let res = GetBirthdayResponse::new("foo", &yesterday_last_year);

        assert!(res.is_ok());

        if let Ok(res) = res {
            assert!(res
                .message
                .starts_with("Hello, foo! Your birthday is in 36"));
        }
    }
}
