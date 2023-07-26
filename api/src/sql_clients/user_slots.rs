use async_trait::async_trait;
use itertools::Itertools;
use sqlx::{types::chrono::NaiveDateTime, FromRow, MySqlPool};

use crate::{
    domains::{
        data_clients::user_slot_client::{UserSlotClient, UserSlots},
        error::Error,
    },
    sql_clients::sql_helper::create_place_holder,
};

#[async_trait]
impl UserSlotClient for MySqlPool {
    async fn fetch_user_slots(
        &self,
        accounts: &[String],
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<UserSlots>, Error> {
        #[derive(Debug, FromRow)]
        pub struct Row {
            pub email: String,
            pub start: Option<NaiveDateTime>,
        }
        let query = format!(
            r#"
            SELECT
                u.email,
                us.start
            FROM
                t_user u LEFT JOIN t_user_slot us ON u.id = us.user_id
            WHERE
                u.email IN ({})
                and us.start between ? and ?
                and TIME(us.start) between '10:00:00' and '19:30:00'
            ORDER BY
                u.id
            "#,
            create_place_holder(accounts.len())
        );
        let rows: Vec<Row> = accounts
            .iter()
            .fold(sqlx::query_as(&query), |q, email| q.bind(email))
            .bind(start_time)
            .bind(end_time)
            .fetch_all(self)
            .await?;

        let slots = accounts
            .iter()
            .map(|account| {
                let account_slots = rows
                    .iter()
                    .filter(|r| r.email == *account)
                    .flat_map(|r| r.start)
                    .collect_vec();
                UserSlots {
                    account: account.clone(),
                    slots: account_slots,
                }
            })
            .collect_vec();
        Ok(slots)
    }

    async fn confirm_user_slots(
        &self,
        accounts: &[String],
        start_time: NaiveDateTime,
    ) -> Result<(), Error> {
        let mut tx = self.begin().await?;

        // slotを追加する対象のユーザをロック
        let lock_query = format!(
            r#"
            SELECT
                *
            FROM
                t_user
            WHERE
                email IN ({})
            FOR UPDATE
            "#,
            create_place_holder(accounts.len())
        );
        accounts
            .iter()
            .fold(sqlx::query(&lock_query), |q, account| q.bind(account))
            .execute(&mut *tx)
            .await?;

        // コンフリクト確認
        let check_conflicts_query = format!(
            r#"
        SELECT
            1
        FROM
            t_user u INNER JOIN t_user_slot us ON u.id = us.user_id
        WHERE
            u.email IN ({})
            and ABS(TIMESTAMPDIFF(MINUTE, ?, us.start)) < 30
            "#,
            create_place_holder(accounts.len())
        );

        let conflicts: bool = accounts
            .iter()
            .fold(sqlx::query(&check_conflicts_query), |q, email| {
                q.bind(email)
            })
            .bind(start_time)
            .fetch_optional(&mut *tx)
            .await?
            .is_some();
        if conflicts {
            // 既に予定あり
            return Err(Error::Conflicts);
        }

        // slotの更新
        let ins_query = format!(
            r#"
            INSERT INTO t_user_slot (user_id, start)
            SELECT 
                id,
                ?
            FROM t_user u
            WHERE
                u.email IN ({})
        "#,
            create_place_holder(accounts.len())
        );
        accounts
            .iter()
            .fold(sqlx::query(&ins_query).bind(start_time), |q, email| {
                q.bind(email)
            })
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
