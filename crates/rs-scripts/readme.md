# rs-scripts:

- 使用 rust 编写的一些脚本工具

> 脚本列表:

- [x] ETH 链上交易查询
- [x] Binance 交易所个人交易数据查询/K 线数据查询
    - 支持 api key 方式下单/撤单

## 脚本功能集:

> binance api:

```ruby


# 市场行情数据查看:
task run:binance:market

# 个人用户数据查看: env 添加 币安的 api key 和 secret
task run:binance

```

> discord bot:

- 🤖 基于 rust 实现的 discord bot 小工具

```ruby

task run:discord

```

## 用法:

- 💡 `--` 方式: 传参
- ✅ 在项目根目录执行:

```ruby

# 查看 cli 工具的帮助
cargo run --bin rs-scripts -- -h

# 示例传参:
cargo run --bin rs-scripts -- -n "Henry"
```

> cli 示例:

- https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs

## notes:

- ✅ 包名: 通常是 - 分隔符(官方 cargo 使用), 少数用 _ 下划线.
- ✅ rust 导包规则, 非常让人迷惑:
    - https://learnku.com/articles/31161
    - https://doc.rust-lang.org/cargo/guide/project-layout.html
    - lib.rs 是很特殊的文件

- ✅ [src/lib.rs](src/lib.rs): 是特殊文件
    - 可以串连整个项目的模块, 对外暴露.

- ✅ `src/bin`: 目录也是特殊目录. ( ❓ ⚠️ ️已经更改为 [bin](bin), bin/ 和 [src](src)/ 平级)
    - https://rustcc.cn/article?id=dcc947c4-21a9-4ba0-ba59-43f6b580aae6
    - ⚠️ rust 隐含规则太坑爹
    - ⚠️ bin/ 目录, 是独立/隔离的 crate 包, 不可直接访问上层目录.
    - ⚠️ 需要借助 src/lib.rs 文件. 这是 hack 做法, 不值得使用.
    - ⚠️ 妥协做法: src/bin/下写模块, 局部导入模块, 而不要在上层组织目录.
    - ⚠️ rust 这个设计, 有点过度设计.
    - ⚠️ src/bin 内, 应该可以单向访问外层目录, 而不允许外部访问内部.

```ruby

源代码在src目录中。
默认库文件是src/lib.rs.
默认的可执行文件是src/main.rs.
其他可执行文件可以放在src/bin/.

```

## reference:

- ✅ https://github.com/topics/rust

> 遵循的目录规范:

- https://github.com/paritytech/substrate/blob/master/bin/node/cli/bin/main.rs
- 👏 参考 `substrate` 的 bin/ 目录规范.
- ✅ 非rust官方做法, rust官方把 bin/ 放在 src/bin/ 下, 显然不恰当.
    - 语义混乱. 混淆 `lib` 和 `bin app` .
    - 不清晰, 且不直观.
- ✅ `substrate` 的做法, 更符合直觉.

> monorepo 风格:

- 👏 包名选择 `-` 分隔, 而非 `_` 分隔.
- ✅ https://github.com/rust-lang/cargo
    - 包名: 官方是 `-` 分割符
- ✅ https://github.com/paritytech/substrate
    - 包名: `-` 分割符
- ✅ https://github.com/swc-project/swc
    - 包名: `_`下划线风格
- ✅ https://github.com/nushell/nushell
    - 混用
- ✅ https://github.com/tauri-apps/tauri
    - 包名: `-` 分割符
- ✅ https://github.com/AppFlowy-IO/AppFlowy
    - 包名: `-` 分割符

> libs:

- ✅ https://github.com/clap-rs/clap
    - 命令行参数解析

> algorithms:

- https://github.com/TheAlgorithms/Rust

> 应用项目:

- https://github.com/solana-labs/solana
- https://github.com/valeriansaliou/sonic
    - 目录结构
    - https://github.com/valeriansaliou/sonic/blob/master/src/main.rs#L130

> binance api:

- https://binance-docs.github.io/apidocs/spot/cn/#185368440e
- https://github.com/binance/binance-spot-api-docs

> terminal ui:

- https://github.com/fdehau/tui-rs
- https://github.com/tarkah/tickrs
