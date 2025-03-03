use lazy_static::lazy_static;
use std::net::SocketAddr;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .target(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    let req = volo_gen::volo::example::GetItemRequest { id: 1024 };

    let resp = CLIENT.clone().get_item(req).await;

    println!("run rpc client: {:?}", resp);

    match resp {
        Ok(info) => tracing::info!("rpc response: {:?}", info),
        Err(e) => tracing::error!("rpc response error: {:?}", e),
    }
}
