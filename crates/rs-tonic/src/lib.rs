#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod hello_world {
    // TODO X: Clion 对此宏展开的识别, 依赖 workspace 为根目录, 否则无法展开跳转
    tonic::include_proto!("helloworld");
}

// way2:
pub mod pb;
pub use pb::helloworld::*;

// mod gen {
//     volo::include_service!("volo_gen.rs");
// }
//
// pub use gen::volo_gen::*;
