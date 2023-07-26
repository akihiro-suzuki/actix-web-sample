use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_new::new;
#[cfg(test)]
use mockall::automock;

use crate::domains::error::Error;

#[derive(Debug, new)]
pub struct UserSlots {
    pub account: String,
    pub slots: Vec<chrono::NaiveDateTime>,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserSlotClient {
    async fn fetch_user_slots(
        &self,
        accounts: &[String],
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<UserSlots>, Error>;

    async fn confirm_user_slots(
        &self,
        accounts: &[String],
        start_time: NaiveDateTime,
    ) -> Result<(), Error>;
}
