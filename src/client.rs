use crate::greeter::CreateShortUrlRequest;
use greeter::greeter_client::GreeterClient;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreateShortUrlRequest {
        full_url: "https://www.youtube.com/".into(),
    });

    println!("Sending request to gRPC Server...");
    let response = client.create_short_url(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
