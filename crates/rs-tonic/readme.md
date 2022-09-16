# Tonic 学习 & 使用:

- https://github.com/hyperium/tonic

> 示例:

- https://github.com/hyperium/tonic/blob/master/examples/README.md

> Clion 配置:
>

- 如果安装 tonic 文档提示, 配置了宏 `tonic::include_proto!()` 代码展开+跳转有问题.
- 先清除 Clion 索引, 重建索引, 重新打开工程, 就正常了.

```rust


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

```
