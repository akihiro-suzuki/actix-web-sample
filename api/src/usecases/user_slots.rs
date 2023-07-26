use std::sync::Arc;

use chrono::NaiveDateTime;
use itertools::Itertools;

use crate::domains::{
    data_clients::user_slot_client::UserSlotClient,
    error::Error,
    slot::{collect_slot_ranges, Slot},
    slot_range::intersect_slot_ranges_array,
};

pub struct UserSlotUsecase {
    pool: Arc<dyn UserSlotClient>,
}
impl UserSlotUsecase {
    pub fn new(pool: Arc<dyn UserSlotClient>) -> Self {
        Self { pool }
    }
    pub async fn fetch_confirmable_slots(
        &self,
        accounts: &[String],
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Result<Vec<Slot>, Error> {
        let user_slots = self
            .pool
            .fetch_user_slots(accounts, start_time, end_time)
            .await?;
        if user_slots.iter().any(|us| us.slots.is_empty()) {
            // 一つもスロットがないユーザがいる場合は空になる
            return Ok(vec![]);
        }

        let slots_list = user_slots
            .into_iter()
            .map(|us| us.slots.into_iter().map(Slot::new).collect_vec())
            .map(|x| collect_slot_ranges(&x))
            .collect_vec();

        let intersected_slots = intersect_slot_ranges_array(slots_list)
            .into_iter()
            .flat_map(|sr| sr.to_slots())
            .sorted_by_key(|x| x.start_date)
            .dedup()
            .collect_vec();

        Ok(intersected_slots)
    }

    pub async fn confirm_users_slot(
        &self,
        accounts: &[String],
        start_time: NaiveDateTime,
    ) -> Result<(), Error> {
        self.pool.confirm_user_slots(accounts, start_time).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::domains::data_clients::user_slot_client::{MockUserSlotClient, UserSlots};

    use super::*;

    fn to_date(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap()
    }
    #[test]
    fn test_fetch_confirmable_slots1() {
        let mut mock = MockUserSlotClient::new();
        mock.expect_fetch_user_slots()
            .times(1)
            .returning(|_, _, _| {
                let us1 = UserSlots::new(
                    "test1@example.com".to_string(),
                    vec![
                        to_date("2020-01-01 10:00:00"),
                        to_date("2020-01-01 10:30:00"),
                        to_date("2020-01-01 11:00:00"),
                        to_date("2020-01-01 13:00:00"),
                        to_date("2020-01-01 13:30:00"),
                    ],
                );
                let us2 = UserSlots::new(
                    "test2@example.com".to_string(),
                    vec![
                        to_date("2020-01-01 10:00:00"),
                        to_date("2020-01-01 10:30:00"),
                        to_date("2020-01-01 13:15:00"),
                        to_date("2020-01-01 14:30:00"),
                    ],
                );
                
                Ok(vec![us1, us2])
            });

        let uc = UserSlotUsecase::new(Arc::new(mock));
        let accounts = vec![
            "test1@example.com".to_string(),
            "test2@example.com".to_string(),
        ];
        let start_time = to_date("2020-01-01 10:00:00");
        let end_time = to_date("2020-01-01 20:00:00");
        let slots = futures::executor::block_on(
            uc.fetch_confirmable_slots(&accounts, start_time, end_time),
        )
        .unwrap();
        assert_eq!(slots.len(), 3);
        assert_eq!(slots[0].start_date, to_date("2020-01-01 10:00:00"));
        assert_eq!(slots[1].start_date, to_date("2020-01-01 10:30:00"));
        assert_eq!(slots[2].start_date, to_date("2020-01-01 13:15:00"));
    }
    #[test]
    fn test_fetch_confirmable_slots_blank() {
        let mut mock = MockUserSlotClient::new();
        mock.expect_fetch_user_slots()
            .times(1)
            .returning(|_, _, _| {
                let us1 = UserSlots::new(
                    "test1@example.com".to_string(),
                    vec![
                        to_date("2020-01-01 10:00:00"),
                        to_date("2020-01-01 10:30:00"),
                        to_date("2020-01-01 11:00:00"),
                        to_date("2020-01-01 13:00:00"),
                        to_date("2020-01-01 13:30:00"),
                    ],
                );
                let us2 = UserSlots::new("test2@example.com".to_string(), vec![]);
                
                Ok(vec![us1, us2])
            });

        let uc = UserSlotUsecase::new(Arc::new(mock));
        let accounts = vec![
            "test1@example.com".to_string(),
            "test2@example.com".to_string(),
        ];
        let start_time = to_date("2020-01-01 10:00:00");
        let end_time = to_date("2020-01-01 20:00:00");
        let slots = futures::executor::block_on(
            uc.fetch_confirmable_slots(&accounts, start_time, end_time),
        )
        .unwrap();
        assert_eq!(slots.len(), 0);
    }
}
