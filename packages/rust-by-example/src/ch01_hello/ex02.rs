use std::fmt::{self, Display, Formatter};

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

#[test]
fn ex02_03_list() {
    use std::fmt;

    // 定义: 单元素简写
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            // start:
            write!(f, "[")?;

            // content:
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                // write!(f, "{}", v)?; // fmt1:
                write!(f, "{}: {}", count, v)?; // fmt2:
            }

            // end:
            write!(f, "]")
        }
    }

    //
    let v = List(vec![1, 2, 3]);
    println!("List {}", v);
}

#[test]
fn ex02_04_fmt() {
    let foo: u32 = 3735928559;
    println!("{}", foo);
    println!("0x:{:X}", foo);
    println!("0o{:o}", foo);

    //////////////////////////////////////////

    // 定义:
    struct City {
        name: &'static str,
        lat: f32, // 纬度
        lon: f32, // 经度
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
            let lon_c = if self.lon >= 0.0 { "E" } else { "W" };

            //
            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    //////////////////////////////////////////

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    //////////////////////////////////////////

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    //
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!(
            "color: {:?}, ({}, {}, {})",
            *color, color.red, color.green, color.blue
        );
    }
}
