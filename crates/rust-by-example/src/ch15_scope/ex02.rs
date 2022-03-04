#[test]
fn ex02_move() {
    /// todo x: 转移所有权
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
    }

    let x = 5u32;
    let y = x;
    println!("x = {}, y= {}", x, y);

    let a = Box::new(5i32);

    println!("a contains {}", a);

    let b = a;
    // error[E0382]
    // println!("a contains: {}", a);

    destroy_box(b);

    // error[E0382]
    // println!("b contains {}", b);
}

#[test]
fn ex02_01_move_mut() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // error[E0594]:
    // *immutable_box = 4;

    // move:
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    // 修改:
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}

#[test]
fn ex02_02_partial_move() {
    ///
    /// todo x: 部分移动
    ///

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    ////////////////////////////////////////////////////////////////////////////////

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // ref: 引用
    let Person { name, ref age } = person;
    println!("The person's name is {} and age is {}", name, age);

    // error: 已部分移动, person 不可用
    // println!("person is {:?}", person);
    println!("The person's age is {:?}", person.age);
}
