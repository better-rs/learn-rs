// todo x: way1, 依赖宏展开补全
use rs_tonic::gen::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest { name: "Tonic".into() });

    let response = client.say_hello(request).await?;

    println!("🚀 tonic client: rpc response = {:?}", response);

    Ok(())
}
