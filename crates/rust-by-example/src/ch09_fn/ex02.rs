use std::iter::Iterator;
use std::mem;

#[test]
fn ex02_closures() {
    // 普通自增加法:
    fn function(i: i32) -> i32 {
        i + 1
    }

    // TODO X: 闭包写法1:
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // TODO X: 闭包写法2: 简写
    let closure_inferred = |i| i + 1;

    ////////////////////////////////////////////////////////////////////////////////

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    ////////////////////////////////////////////////////////////////////////////////

    // TODO X: 无参闭包,自动推导返回值类型
    let one = || 1;
    println!("closure returning one: {}", one());
}

// 捕获:
#[test]
fn ex02_01_capture() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);
    print();

    // 只读引用:
    let _reborrow = &color;
    print();

    // 赋值:
    let _color_moved = color;
    // print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    // let _reborrow = &count;
    inc();

    let _count_reborrowed = &mut count;

    ////////////////////////////////////////////////////////////////////////////////

    // 被闭包捕获:
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);

        // TODO X: 闭包, 捕获外部变量
        mem::drop(movable);
    };

    consume();
    // TODO X: error
    // consume();

    ////////////////////////////////////////////////////////////////////////////////

    let haystack = vec![1, 2, 3];

    // TODO X: move + 闭包, 强制获取所有权
    let contains = move |needle| haystack.contains(needle);

    println!("contains: {}", contains(&1));
    println!("contains: {}", contains(&4));

    // error: value moved
    // println!("`haystack`: {}", haystack.len());
}

#[test]
fn ex02_02_input_parameters() {
    /*
    TODO X:
        Fn：表示捕获方式为通过引用（&T）的闭包 // 只读引用
        FnMut：表示捕获方式为通过可变引用（&mut T）的闭包 // 可变引用
        FnOnce：表示捕获方式为通过值（T）的闭包 // 值获取
    */

    // TODO X: 值获取
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    // TODO X: 闭包函数做参数
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    ////////////////////////////////////////////////////////////////////////////////

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // todo x:
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

#[test]
fn ex02_03_anonymity() {
    // TODO X: 闭包函数做参数
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    // TODO X: 闭包函数做参数
    fn apply2<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 7;
    let print = || println!("x={}", x);

    apply(print);
    apply2(print);
}

// 函数参数:
#[test]
fn ex02_04_input_functions() {
    // TODO X: 闭包函数做参数
    fn call_me<F: Fn()>(f: F) {
        f()
    }

    fn function() {
        println!("I'm a function!");
    }

    ////////////////////////////////////////////////////////////////////////////////

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}

// 函数返回值:
#[test]
fn ex02_05_output_parameters() {
    /*
    TODO X:
        返回闭包的有效特征是：
            Fn
            FnMut
            FnOnce
        除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过值进行的。

    */

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        // TODO X: Move+闭包 => 返回值
        move || println!("This is a: {}", text)
    }

    fn create_fn_mut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        // TODO X: Move+闭包 => 返回值
        move || println!("This is a: {}", text)
    }

    fn create_fn_once() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        // TODO X: Move+闭包 => 返回值
        move || println!("This is a: {}", text)
    }

    ////////////////////////////////////////////////////////////////////////////////

    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    // TODO X: call
    fn_plain();
    fn_mut();
    fn_once();
}

#[test]
fn ex02_06_01_iter_any() {
    /*
    TODO X:
        Iterator::any 是一个函数，若传给它一个迭代器（iterator），
        当其中任一元素满足谓词（predicate）时它将返回 true，否则返回 false

    TODO X:
        pub trait Iterator {
            type Item;
            fn any<F>(&mut self, f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool,
            {
                f()
            }
        }
    */

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // TODO X: iter + &x
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // TODO X: into_iter + x
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}

#[test]
fn ex02_06_02_iter_find() {
    /*
    TODO X:
        Iterator::find 是一个函数，在传给它一个迭代器时，将用 Option 类型返回第一个满足谓词的元素。
        pub trait Iterator {
            type Item;
            fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
            where
                P: FnMut(&Self::Item) -> bool,
            {
                predicate
            }
        }

    */

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // TODO X:
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}
