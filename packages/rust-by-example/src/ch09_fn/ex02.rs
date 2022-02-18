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
fn ch02_01_capture() {
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
