use std::{env, path::PathBuf};

fn main() {
    //
    // TODO X: 会把 pb.rs 代码生成到: target/debug/build/rs-tonic-xxxx/out 目录下.
    //      使用宏 tonic::include_proto!() 动态导入
    //
    tonic_build::compile_protos("proto/helloworld/helloworld.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    //
    // todo x: 生成到本项目内: src/pb/
    //      - ref: https://github.com/tyrchen/rust-training/blob/master/live_coding/tonic-live/build.rs
    //
    tonic_build::configure()
        // .type_attribute(".", "#[derive(Hash, Eq, serde::Serialize, serde::Deserialize)]")
        .out_dir("src/pb")
        .compile(&["./proto/helloworld/helloworld.proto"], &["./proto/"])
        .unwrap();

    //
    //
    //
    println!("cargo:rerun-if-changed=./proto/helloworld.proto");
    println!("cargo:rerun-if-changed=./build.rs");

    build_json_codec_service();
}

fn main2() {
    // tonic_build::configure()
    //     .type_attribute("routeguide.Point", "#[derive(Hash)]")
    //     .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
    //     .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    // tonic_build::configure()
    //     .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
    //     .server_attribute("Echo", "#[derive(PartialEq)]")
    //     .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
    //     .client_attribute("Echo", "#[derive(PartialEq)]")
    //     .compile(&["proto/attrs/attrs.proto"], &["proto"])
    //     .unwrap();

    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["proto/googleapis/google/pubsub/v1/pubsub.proto"], &["proto/googleapis"])
    //     .unwrap();

    build_json_codec_service();
}

// Manually define the json.helloworld.Greeter service which used a custom JsonCodec to use json
// serialization instead of protobuf for sending messages on the wire.
// This will result in generated client and server code which relies on its request, response and
// codec types being defined in a module `crate::common`.
//
// See the client/server examples defined in `src/json-codec` for more information.
fn build_json_codec_service() {
    let greeter_service = tonic_build::manual::Service::builder()
        .name("Greeter")
        .package("json.helloworld")
        .method(
            tonic_build::manual::Method::builder()
                .name("say_hello")
                .route_name("SayHello")
                .input_type("crate::common::HelloRequest")
                .output_type("crate::common::HelloResponse")
                .codec_path("crate::common::JsonCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new().compile(&[greeter_service]);
}
