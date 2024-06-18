use super::validation::ValidatedUsername;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserBirthdayRequest {
    pub date_of_birth: String,
}

pub(crate) async fn upsert_user(username: ValidatedUsername, req: UserBirthdayRequest) {
    let ValidatedUsername(username) = username;
    log::info!("user: {}, dob: {:?}", username, req);
}

pub(crate) async fn get_birthday() {
    // let username = req.param("username").unwrap();
    // let birthday = db.get_birthday(username).await;
    // Ok(birthday)
}
