# Rust Core:

- Rust 核心语法点

## 基础:

- `# [test]`: 用于标记测试方法
- `#[derive(Debug)]`: 打印输出标记
- `use std::fmt;` // 导入 `fmt`

```ruby

fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推导（derive，即自 动创建）
fmt::Debug 的实现。但是 fmt::Display 需要手动实现。

fmt::Debug 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。
这需要通过 手动实现 fmt::Display 来做到。fmt::Display 采用 {} 标记。
Rust 也通过 {:#?} 提供了 “美化打印” 的功能：
通过手动实现 fmt::Display 来控制显示效果。

`{:b}` 需要实现 `fmt::Binary`


// 该属性用于隐藏对未使用代码的警告。
// 可以用于标记整个函数，或者某个变量。
#![allow(dead_code)]


// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]


```


