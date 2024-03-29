# Tonic 学习 & 使用:

- https://github.com/hyperium/tonic

> 示例:

- https://github.com/hyperium/tonic/blob/master/examples/README.md
- https://github.com/tyrchen/rust-training/blob/master/live_coding/tonic-live/build.rs#L7
    - 陈天写的 demo

## Run:

- ✅ 本repo 给了 pb 代码2种使用示例. 一种是静态生成pb代码导入, 一种是宏导入, 依赖clion补全展开.
    - ✅ [helloworld](bin/helloworld)
    - ✅ [helloworld2](bin/helloworld2)
- ✅ 两种方式都可正常使用, 建议使用`显式生成pb + 导入`的方式, 不依赖 IDE 补全能力.
    - 因为 vscode + ra 当前还不具备宏导入时的补全能力.

```ruby

# root directory:
cd learn-rs/

# run rpc server:
task tonic:run

# run rpc client:
task tonic:run:client

```

## Clion 配置 宏 tonic::include_proto!()

> Clion 配置:

- 如果安装 tonic 文档提示, 配置了宏 `tonic::include_proto!()` 代码展开+跳转有问题.
- 先清除 Clion 索引, 重建索引, 重新打开工程, 就正常了.

```rust


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

```
