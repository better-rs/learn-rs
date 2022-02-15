#[test]
fn ex02_enum() {
    enum WebEvent {
        PageLoad,
        PageUnload,

        KeyPress(char),
        Paste(String),

        // 结构体
        Click { x: i64, y: i64 },
    }

    // 匹配枚举:
    fn inspect(event: WebEvent) {
        // 匹配写法:
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted '{}'.", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    //
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    ////////////////////////////////////////////////////////////////////////////////

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // 别名:
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let x = Operations::Add;
    let y = Operations::Subtract;

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    println!("add= {}, sub = {}", x.run(20, 333), y.run(20, 333));
}

// 3.2.1 // 使用 use 关键字
#[test]
#[allow(dead_code)] // 该属性用于隐藏对未使用代码的警告。
fn ex02_01_enum_use() {
    #[allow(dead_code)] // 可以标记类型定义
    enum Status {
        Rich,
        Poor,
    }

    #[allow(dead_code)] // 可以标记类型定义
    enum Work {
        Civilian,
        Soldier,
    }

    ////////////////////////////////////////////////////////////////////////////////

    // 导入模块:
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldier fight!"),
    }
}

// 3.2.2 // C 风格用法
#[allow(dead_code)]
#[test]
fn ex02_02_c_like() {
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32); // 类型转换
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violet are #{:06x}", Color::Blue as i32);
}

#[test]
fn ex02_03_linked_list() {
    use List::*;

    enum List {
        // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
        Cons(u32, Box<List>),
        Nil, // end of the list
    }

    impl List {
        // 创建一个空 list
        fn new() -> List {
            Nil
        }

        //
        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            // 匹配:
            match self {
                // 不能得到 tail 的所有权，因为 `self` 是借用的；
                // 因此使用一个对 tail 的引用
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match self {
                Cons(head, ref tail) => {
                    // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                    // 而不是打印结果到控制台上
                    format!("{},{}", head, tail.stringify())
                }

                Nil => {
                    format!("Nil")
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
