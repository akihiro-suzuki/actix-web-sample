use std::collections::HashMap;

use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{FromRow, MySqlPool};

use crate::domains::{data_clients::test_client::TestClient, error::Error};

#[async_trait]
impl TestClient for MySqlPool {
    async fn dump_data(&self) -> Result<HashMap<String, String>, Error> {
        #[derive(Debug, FromRow, Deserialize)]
        struct Row {
            email: String,
            starts: Option<String>,
        }
        let rows: Vec<Row> = sqlx::query_as!(
            Row,
            r#"
            SELECT
                u.email,
                CONCAT('[', GROUP_CONCAT(DATE_FORMAT(us.start, '%Y/%m/%d %H:%i') order by us.start), ']') as starts
            FROM
                t_user u INNER JOIN t_user_slot us ON u.id = us.user_id
            GROUP BY
                u.id
            ORDER BY
                u.id
            "#
        )
        .fetch_all(self)
        .await?;

        let mut map = HashMap::new();
        for row in rows.into_iter().filter(|r| r.starts.is_some()) {
            map.insert(row.email, row.starts.unwrap());
        }
        Ok(map)
    }

    async fn clear_data(&self) -> Result<(), Error> {
        let mut tx = self.begin().await?;
        sqlx::query!("DELETE FROM t_user_slot")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
