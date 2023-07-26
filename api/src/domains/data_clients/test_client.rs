use std::collections::HashMap;

use async_trait::async_trait;

use crate::domains::error::Error;

#[async_trait]
pub trait TestClient {
    async fn dump_data(&self) -> Result<HashMap<String, String>, Error>;
    async fn clear_data(&self) -> Result<(), Error>;
}
