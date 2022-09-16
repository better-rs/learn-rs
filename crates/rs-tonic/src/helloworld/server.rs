use tonic::{transport::Server, Request, Response, Status};

use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("🚀 tonic rpc,  Got a request from {:?}", request.remote_addr());

        let reply =
            hello_world::HelloReply { message: format!("Hello {}!", request.into_inner().name) };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("🚀 GreeterServer listening on {}", addr);

    Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;

    Ok(())
}
