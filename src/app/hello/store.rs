use crate::app::Store;
use anyhow::Result;
use chrono::NaiveDate;

static BIRTHDAY_NS: &str = "birthday";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(super) struct Birthday {
    dob: NaiveDate,
}

pub(super) trait BirthdayStore {
    async fn get_birthday(&self, username: &str) -> Result<Option<Birthday>>;
    async fn upsert_birthday(&self, username: String, dob: NaiveDate) -> Result<()>;
}

impl BirthdayStore for Store {
    async fn get_birthday(&self, username: &str) -> Result<Option<Birthday>> {
        let record: Option<Birthday> = self.db.select((BIRTHDAY_NS, username)).await?;

        Ok(record)
    }

    async fn upsert_birthday(&self, username: String, dob: NaiveDate) -> Result<()> {
        let _record: Option<Birthday> = self
            .db
            .update((BIRTHDAY_NS, &username))
            .content(Birthday { dob })
            .await?;

        Ok(())
    }
}
