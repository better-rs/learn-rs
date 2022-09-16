#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub use hello_world::*;

// mod gen {
//     volo::include_service!("volo_gen.rs");
// }
//
// pub use gen::volo_gen::*;
