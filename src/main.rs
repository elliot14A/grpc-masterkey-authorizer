#![allow(dead_code)]

mod auth_key;
mod grpc;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting grpc server...");
    grpc::start_grpc_server().await
}
