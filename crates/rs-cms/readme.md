# Rust CMS:

- 基于 [axum](https://github.com/tokio-rs/axum) 和 [tokio](https://github.com/tokio-rs/tokio) 开发的 CMS 系统.

## 功能:

- 账号系统: 注册/登录
- 内容系统: 创建/编辑/删除/查看内容
- 评论系统: 创建/编辑/删除/查看评论
- Admin 系统: 后台管理

## requirements:

- Rust 1.60.0+
- go-task
- https://github.com/watchexec/cargo-watch

> 模块:

- ORM:
- DB:
- Cache: Redis

## Quick Start:

```rust shell   

task run

```

## reference:

- https://github.com/tokio-rs/axum
- https://github.com/tokio-rs/tokio
- https://github.com/rust-lang/mdBook
- https://github.com/getzola/zola

> 参考 CMS:

- https://github.com/LemmyNet/lemmy
    - rust 实现的类似 reddit 的社区系统
- https://github.com/TianLangStudio/rust_cms

> docs:

- ORM 使用:
    - https://diesel.rs/guides/getting-started
    - https://diesel.rs/guides/all-about-inserts.html
    - https://www.rectcircle.cn/posts/rust-diesel/
    - https://learnku.com/docs/rust-irwa/create-database-access-layer/10353
    - https://www.cnblogs.com/rongfengliang/p/12256614.html
    - https://www.ancii.com/ab3q5edb5/
    - https://github.com/diesel-rs/diesel/blob/master/examples/mysql/getting_started_step_1/src/schema.rs
    