use crate::models::{AuthKey, CreateAuthKey};

use super::AuthKeyTrait;

pub struct HeedAuthKeyImpl {}

#[tonic::async_trait]
impl AuthKeyTrait for HeedAuthKeyImpl {
    async fn create_key(&self, input: CreateAuthKey) -> AuthKey {
        AuthKey {
            name: input.name,
            description: input.description,
            token: "test token".into(),
        }
    }
}
