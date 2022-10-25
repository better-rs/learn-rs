# Rust + SQLCipher:

- [SQLCipher]() 是 [Sqlite]() 的加密版本(fork 版本)

## quickstart:

- 初始化:

```ruby
cd crates/rs-sqlcipher/

# 创建 tmp, 创建db文件
task init

```

- 运行:

```ruby

cd crates/rs-sqlcipher/

# run:
task run


```

## 搭配 rust 使用:

> 依赖:

- [sqlx](https://github.com/launchbadge/sqlx)
- [libsqlite3-sys](https://crates.io/crates/libsqlite3-sys)

```toml
[dependencies]
libsqlite3-sys = { version = "0.25", features = ["bundled-sqlcipher"] }

# Tokio:
sqlx = { version = "0.6", features = ["runtime-tokio-native-tls", "sqlite"] }
tokio = { version = "1", features = ["full"] }


```

> sqlite url:

- 通过环境变量注入

```rust

/// | URL | Description |
/// | -- | -- |
/// `sqlite::memory:` | Open an in-memory database. |
/// `sqlite:data.db` | Open the file `data.db` in the current directory. |
/// `sqlite://data.db` | Open the file `data.db` in the current directory. |
/// `sqlite:///data.db` | Open the file `data.db` from the root (`/`) directory. |
/// `sqlite://data.db?mode=ro` | Open the file `data.db` for read-only access. |
///

```


## 加密参数: 


- https://github.com/launchbadge/sqlx/blob/main/tests/sqlite/sqlcipher.rs#L51



## 使用说明:

> 使用场景:

- desktop app 数据库: 加密数据存储
- mobile app 数据库: 加密数据存储

> 使用案例:

- 微信 app 的聊天数据, 使用 [SQLCipher]() 方案实现.
- 大量应用都在使用.

> 特别提醒:

- 此加密方案, 防君子, 不防小人.
- 并不能应对破解(原理上, 是比较容易破解的)
- 但是可以规避大多数普通场景的数据安全, 应对普通用户, 不能随意获取`加密数据`.



## reference:

- https://github.com/rusqlite/rusqlite/issues/926
