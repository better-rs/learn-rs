# rs-scripts:

- 使用 rust 编写的一些脚本工具

## notes:

- 包名: 通常是 - 分隔符(官方 cargo 使用), 少数用 _ 下划线.
- rust 导包规则, 非常让人迷惑:
    - https://learnku.com/articles/31161
    - https://doc.rust-lang.org/cargo/guide/project-layout.html
    - lib.rs 是很特殊的文件


```ruby

源代码在src目录中。
默认库文件是src/lib.rs.
默认的可执行文件是src/main.rs.
其他可执行文件可以放在src/bin/.


```

## 用法:

- `--` 方式: 传参

- 在项目根目录执行:

```ruby

# 查看 cli 工具的帮助
cargo run --bin rs-scripts -- -h  

# 示例传参:
cargo run --bin rs-scripts -- -n "Henry"
```

> cli 示例:

- https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs

## reference:

- https://github.com/topics/rust

> monorepo 风格:

- https://github.com/rust-lang/cargo
    - monorepo 风格
    - 包名: 官方是 - 分割符
- https://github.com/swc-project/swc
    - monorepo 风格
    - 包名: 下划线风格
- https://github.com/nushell/nushell
    - monorepo 风格
- https://github.com/tauri-apps/tauri
- https://github.com/AppFlowy-IO/AppFlowy

> libs:

- https://github.com/clap-rs/clap

> algorithms:

- https://github.com/TheAlgorithms/Rust

> 应用项目:

- https://github.com/solana-labs/solana
- https://github.com/valeriansaliou/sonic
    - 目录结构
    - https://github.com/valeriansaliou/sonic/blob/master/src/main.rs#L130