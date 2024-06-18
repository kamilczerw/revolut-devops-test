#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserBirthdayRequest {
    pub date_of_birth: String,
}

pub(crate) async fn upsert_user(req: UserBirthdayRequest) {
    log::info!("dob: {:?}", req);
}

pub(crate) async fn get_birthday() {
    // let username = req.param("username").unwrap();
    // let birthday = db.get_birthday(username).await;
    // Ok(birthday)
}
