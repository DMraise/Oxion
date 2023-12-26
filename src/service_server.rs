use tonic::{transport::Server, Request, Responce, Status};

pub mod my_service {
    tonic::include_proto!("my_grpc");
}

use my_grpc::my_service_server::{MyServiceServer, MyService};
use my_grpc::{MyRequest, MyResponce};

#[derive(Default)]
pub struct MyServiceImpl;

#[tonic::async_trait]
impl MyService for MyServiceImpl {
    async fn my_method(&self, request: Request:<MyRequest>) -> Result<Responce<MyResponce>, Status> {
        let responce = MyResponce {
            result: format!("Reeived: {}", request.get_ref().data),
        };
        Ok(Responce::new(responce))
    }
}

// #[tokio::main]
async fn run_tokio() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_serivce_impl = MyServiceImpl::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(MyServiceServer::new(my_serivce_impl))
        .server(addr)
        .await?;

    Ok(())
}