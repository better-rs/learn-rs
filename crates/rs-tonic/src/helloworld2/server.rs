use tonic::{transport::Server, Request, Response, Status};

// todo x: way2, 静态导入 pb 代码
use rs_tonic::pb::helloworld::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("🚀 tonic rpc,  Got a request from {:?}", request.remote_addr());

        let reply = HelloReply { message: format!("Hello {}!", request.into_inner().name) };
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
