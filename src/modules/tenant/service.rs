use std::sync::Arc;

use crate::error::ApiError;

use super::{port::DBRepository, Tenant};

pub struct Service {
    db_port: Arc<dyn DBRepository>,
}
impl Service {
    pub fn new(db_port: Arc<dyn DBRepository>) -> Self {
        Self { db_port }
    }

    pub async fn find_by_user_id(&self, id: &str) -> Result<Tenant, ApiError> {
        self.db_port.find_by_user_id(id).await
    }
}
