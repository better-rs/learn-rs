//
#[test]
fn ex02() {
    println!("type=i32, {} days", 2233); // 默认 type = i32
    println!("type=i32, {} days", 22i32); // type = i32
    println!("type=i64, {} days", 1234i64); // type = i64

    // fmt:
    println!("{0}, this is {1}. {1}, this is {0}.", "Tom", "Jimmy",);

    // fmt2:
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 特殊格式:
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 文本对齐:
    println!("{number:>width$}", number = 1, width = 6); // 补空白
    println!("{number:>0width$}", number = 1, width = 6); // 补0

    // 参数个数匹配:
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // 结构体:
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
}
