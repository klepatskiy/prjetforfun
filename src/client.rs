use greeter::greeter_client::GreeterClient;
use greeter::GetUrlRequest;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetUrlRequest {
        short_url: "Tonic".into(),
    });

    println!("Sending request to gRPC Server...");
    let response = client.get_url(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
