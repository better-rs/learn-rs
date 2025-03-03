#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use std::net::SocketAddr;

use volo_example::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    println!("run rpc server: {}", addr);

    volo_gen::volo::example::ItemServiceServer::new(S).run(addr).await.unwrap();
}
