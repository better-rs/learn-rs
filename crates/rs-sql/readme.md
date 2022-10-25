# sqlx 用例:

- [sqlx](https://github.com/launchbadge/sqlx) 使用示例代码.

## packages:

- ✅ [rs-sqlcipher](rs-sqlcipher): rust + sqlx + sqlite 示例.

## 说明:

- sqlx 支持异步, 对数据库支持比较全. 使用多, bug 少.

## `Raw SQL`  vs `ORM`:

- rust 当前 `ORM` 使用体验一般, 而且大多数场景, 使用 `ORM`,远远不如 `raw sql` 更灵活+高效.
- 大厂的日常业务开发, 也基本是 `raw sql`, 几乎不使用ORM, 原因主要是 性能开销.
- ORM, 多在 `admin API` 开发中使用, 这些场景, 对性能不敏感.

## RUST ORM:

- [sea-orm](https://github.com/SeaQL/sea-orm)
    - 基于 sqlx, 支持异步
- [diesel](https://github.com/diesel-rs/diesel)
    - 不支持异步
- [rbatis](https://github.com/rbatis/rbatis)
    - 国人项目

> 综合评价:

- 整体使用感受, 体验一般.
- 没有特别舒服的. 可能不如直接使用 `sqlx`
