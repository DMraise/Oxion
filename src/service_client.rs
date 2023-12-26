use tonic::Request;

pub mod my_service {
    tonic::include_proto!("my_grpc");
}

use my_service::my_service_client::MyServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let channel = tonic::transport::Channel::from_static("http://[..1]:50051")
        .connect()
        .await?;

    let mut client = MyServiceClient::new(channel);

    let request = tonic::Request::new(my_serivce::MyRequest {
        data: "Hello gRPC!".into(),
    });

    let responce = client.my_method(request).await?;

    println!("RESPONSE={:?}", responce);

    Ok(())
}