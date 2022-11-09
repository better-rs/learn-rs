# development:

## Project Structure:

- 使用 monorepo, 参考 rust 官方 cargo 目录结构, 组织代码.
- 详细内容见每个子目录的 readme.md

```ruby

-> % tree ./crates -L 2
./crates
├── basic
│   ├── Cargo.toml
│   └── src
├── rs-cli            // 使用 rust 写的工具脚本
│   ├── Cargo.toml
│   └── src
├── rust-by-example       // rust 官方学习教程 // 练习代码
│   ├── Cargo.toml
│   └── src
├── rust-programming-cookbook // 《Rust 语言编程实战》 // 练习代码
│   ├── Cargo.toml
│   └── src
└── rust-programming-in-action // 《Rust编程：入门、实战与进阶》 // 练习代码
    ├── Cargo.toml
    └── src

10 directories, 5 files


```

## Setup development environment:

- 配置 rust 开发环境
- 安装 Clion

### Clion:

- https://www.jetbrains.com/help/clion/rust-support.html#tests
- https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-testing.html
- 基于 unit test 方式编写代码.
- Clion IDE 支持直接运行 test 代码, 方便快速测试语法点

## Reference:

- https://play.rust-lang.org/
    - 官方练习+分享代码
- https://github.com/rust-unofficial/awesome-rust
- [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/index.html)
- https://github.com/rust-lang/rust-by-example
- https://github.com/rust-lang/rustlings
- https://github.com/PacktPublishing
    - 有很多 rust book 源码

> Rust 目录规范:

- https://github.com/rust-lang/cargo
    - 参考 cargo 的目录结构(monorepo)
- https://github.com/rust-lang/crates.io
    - 参考一个 web 应用项目的目录结构

> backlog:

- https://github.com/PacktPublishing/Mastering-RUST-Second-Edition
- https://github.com/PacktPublishing/Rust-Web-Programming
- https://github.com/PacktPublishing/Hands-On-Microservices-with-Rust
- https://github.com/PacktPublishing/Creative-Projects-for-Rust-Programmers
- https://github.com/PacktPublishing/Speed-up-your-Python-with-Rust
