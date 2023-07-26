use std::{collections::HashMap, sync::Arc};

use crate::domains::{data_clients::test_client::TestClient, error::Error};

pub struct DataUsecase {
    pool: Arc<dyn TestClient>,
}
impl DataUsecase {
    pub fn new(pool: Arc<dyn TestClient>) -> Self {
        Self { pool }
    }
    pub async fn dump(&self) -> Result<HashMap<String, String>, Error> {
        self.pool.dump_data().await
    }
    pub async fn clear(&self) -> Result<(), Error> {
        self.pool.clear_data().await
    }
}
