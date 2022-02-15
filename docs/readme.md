# rust 学习笔记

## rust workspace 规则:

- 只包含 workspace 字段, 不包含其他
- 也不包含包依赖

## rust mod 目录结构规则:

> 命名规范:

1. rust 文件: 字母开头+下划线分割, 不可以数字开头(无法识别)
2. mod.rs: 导入本文件夹其他模块
    1. mod xxx, xxx 对应当前文件夹下其他文件名(注意是文件名)
3.

## rust test:

- `#[test]`: 使用此标记, 标记一个函数, 自动变成 unit test, 可以被 IDE 识别+执行
