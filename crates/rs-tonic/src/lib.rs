#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

/*

TODO X: 同时对比2个 case 的使用差别

*/

//
// TODO X: way1, 依赖宏展开 dynamic, 目前只有 Clion 支持补全
//
pub mod gen {
    // TODO X: Clion 对此宏展开的识别, 依赖 workspace 为根目录, 否则无法展开跳转
    tonic::include_proto!("helloworld");
}

//
// TODO X: way2, 使用显式生成的pb代码, src/pb
//
pub mod pb;
// pub use pb::helloworld::*;
