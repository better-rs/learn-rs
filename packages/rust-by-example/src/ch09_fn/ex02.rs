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
