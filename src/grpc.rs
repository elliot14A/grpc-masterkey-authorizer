use proto::auth_key_server::{AuthKey, AuthKeyServer};
use tonic::transport::Server;

use crate::{
    auth_key::{self, AuthKeyTrait},
    models::{AuthKey as AuthKeyModel, CreateAuthKey},
};

mod proto {
    tonic::include_proto!("auth_key");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("auth_key_descriptor");
}

#[derive(Debug)]
pub struct CreateAuthService<S: AuthKeyTrait> {
    pub svc: S,
}

#[tonic::async_trait]
impl<S: AuthKeyTrait> AuthKey for CreateAuthService<S> {
    async fn create_auth_key(
        &self,
        request: tonic::Request<proto::CreateAuthKeyRequest>,
    ) -> Result<tonic::Response<proto::CreateAuthKeyResponse>, tonic::Status> {
        let body = request.into_inner();
        let response = self.svc.create_key(body.into()).await;
        Ok(tonic::Response::new(response.into()))
    }
}

impl From<proto::CreateAuthKeyRequest> for CreateAuthKey {
    fn from(value: proto::CreateAuthKeyRequest) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

impl From<AuthKeyModel> for proto::CreateAuthKeyResponse {
    fn from(value: AuthKeyModel) -> Self {
        Self {
            api_key: value.token,
        }
    }
}

pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let svc = auth_key::new_auth_key_impl();
    let auth_key_svc = CreateAuthService { svc };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(AuthKeyServer::new(auth_key_svc))
        .serve(addr)
        .await?;

    Ok(())
}
