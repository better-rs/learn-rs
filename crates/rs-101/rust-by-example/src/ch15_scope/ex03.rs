#[test]
fn ex03_borrow() {
    //

    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    ////////////////////////////////////////////////////////////////////////////////

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 引用:
        let _ref_to_i32: &i32 = &boxed_i32;

        //eat_box_i32(boxed_i32); // error: cannot move out of `boxed_i32` because it is borrowed

        // boxed_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}

#[test]
fn ex03_01_borrow_mut() {
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition, by {}", book.title, book.year, book.author);
    }

    // 参数: 可写引用
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    ////////////////////////////////////////////////////////////////////////////////

    let immutabook =
        Book { author: "Douglas Hofstadter", title: "Godel, Escher, Batch", year: 1979 };

    let mut mutabook = immutabook;

    //
    // borrow_book(&immutabook);
    borrow_book(&mutabook);

    // 借用:
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
}

#[test]
fn ex03_02_borrow_alias() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    ////////////////////////////////////////////////////////////////////////////////

    let mut point = Point { x: 0, y: 0, z: 0 };

    // TODO X: 只读引用
    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

    // TODO X: 多次只读借用前, 不允许出现可写借用.
    // let mutable_borrow = &mut point;

    // TODO X: 多次只读借用前, 不允许出现可写借用.
    println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

    // TODO X: 无只读借用, 允许出现可写借用.
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

#[test]
fn ex03_03_borrow_ref() {
    /*
        TODO X:
            在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来创建结构体/元组的 字段的引用。

    */

    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    ////////////////////////////////////////////////////////////////////////////////

    let c = 'Q';

    // TODO X: 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
    let ref ref_c1 = c; // TODO X: 写法1
    let ref_c2 = &c; // TODO X: 写法2

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 解构:
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x, // TODO X: 只读引用
            y: _,
        } = point;

        // TODO X: 返回
        *ref_to_x
    };

    ////////////////////////////////////////////////////////////////////////////////

    let mut mutable_point = Point { x: 0, y: 0 };

    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y, // TODO X: 可写引用
        } = mutable_point;

        // TODO X: 修改值
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    ////////////////////////////////////////////////////////////////////////////////

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple; // TODO X: 可写引用

        // TODO X: 修改值
        *last = 2;
    }

    println!("tuple is {:?}", mutable_tuple);
}
