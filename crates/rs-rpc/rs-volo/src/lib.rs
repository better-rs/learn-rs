#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use std::collections::HashMap;

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_grpc::Request<volo_gen::volo::example::GetItemRequest>,
    ) -> core::result::Result<
        volo_grpc::Response<volo_gen::volo::example::GetItemResponse>,
        volo_grpc::Status,
    > {
        println!("rpc request call");

        let resp = volo_gen::volo::example::GetItemResponse {
            item: Some(volo_gen::volo::example::Item {
                title: "test".to_string(),
                id: 22,
                content: "test content".to_string(),
                extra: HashMap::new(),
            }),
        };

        Ok(volo_grpc::Response::new(resp))
    }
}
