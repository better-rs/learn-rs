#[test]
fn ex04_freeze() {
    let mut x = 7i32;

    //
    {
        // 变量遮蔽:
        let x = x;

        // x = 50;
        println!("immutable_integer is {}", x);

        // _mutable_integer = 50; // error: cannot assign to `_mutable_integer` because it is
        // borrowed
    }

    println!("_mutable_integer is {:?}", x);

    // 重新赋值:
    x = 3;
    println!("_mutable_integer is {:?}", x);
}
