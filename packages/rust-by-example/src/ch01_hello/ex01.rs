// https://rustwiki.org/zh-CN/rust-by-example/hello/comment.html
#[test]
fn ex01() {
    // 这是行注释的例子
    // 注意有两个斜线在本行的开头
    // 在这里面的所有内容都不会被编译器读取

    println!("Hello, world!");

    // 观察块注释是如何简单地对表达式进行修改的，行注释则不能这样。
    // 删除注释分隔符将会改变结果。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
