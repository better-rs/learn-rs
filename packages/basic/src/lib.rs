/*

参考:
    https://rustwiki.org/zh-CN/rust-by-example/index.html

说明:
    1. 根据例子学习Rust语言
    2. 借助 IDE 可以快速执行 test 方法的技巧, 快速学习.

 */


/*
    https://rustwiki.org/zh-CN/rust-by-example/hello/comment.html

*/
#[test]
fn ex01_01() {
    println!("Hello, world!");

    // 观察块注释是如何简单地对表达式进行修改的，行注释则不能这样。
    // 删除注释分隔符将会改变结果。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

// 格式化输出
#[test]
fn ex02_01() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 插值格式化字符串:
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
