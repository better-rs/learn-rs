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

#[test]
fn ex02_01_debug() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // fmt:
    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // print struct:
    println!("Now {:?} will print!", Structure(3));

    // print Deep:
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 结构体定义:
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str, // & = 只读引用 // a' = 生命周期标记
        age: u8,       // 无符号类型
    }

    let name = "Peter";
    let age = 27;
    let perter = Person { name, age };

    println!(
        "person: {:#?}, name:{}, age:{}",
        perter, perter.name, perter.age,
    );
}

#[test]
fn ex02_02_display() {
    use std::fmt;
    use std::fmt::Formatter;

    // 类型定义
    struct Structure(i32);

    // 接口实现:
    impl fmt::Display for Structure {
        // 接口方法实现:
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "struct: {}", self.0) // 注意是表达式,  自动 return
        }
    }

    //
    println!("{}", Structure(2233));

    ////////////////////////////////////////////////////////////////////////

    // 数字类型:
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1) // 注意是表达式,  自动 return
        }
    }

    //
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y) // 注意是表达式,  自动 return
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    //
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "big range: {big}, small range: {small}",
        big = big_range,
        small = small_range
    );

    //
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare Point: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // println!("What does Point2D look like in binary: {:b}?", point);
}
