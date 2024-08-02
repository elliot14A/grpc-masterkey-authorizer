mod heed;
use crate::models;

#[tonic::async_trait]
pub trait AuthKeyTrait: Send + Sync + 'static {
    async fn create_key(&self, input: models::CreateAuthKey) -> models::AuthKey;
}

pub fn new_auth_key_impl() -> heed::HeedAuthKeyImpl {
    heed::HeedAuthKeyImpl {}
}
