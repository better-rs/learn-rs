mod ex01;
mod ex02;
mod ex03;

/*

TODO X:
    属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：
        条件编译代码
        设置 crate 名称、版本和类型（二进制文件或库）
        禁用 lint （警告）
        启用编译器的特性（宏、全局导入（glob import）等）
        链接到一个非 Rust 语言的库
        标记函数作为单元测试
        标记函数作为基准测试的某个部分
    属性可以接受参数，有不同的语法形式：
        #[attribute = "value"]
        #[attribute(key = "value")]
        #[attribute(value)]
    属性可以多个值，它们可以分开到多行中：
        #[attribute(value, value2)]
        #[attribute(value, value2, value3,
                    value4, value5)]

*/
