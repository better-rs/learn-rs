#[test]
#[allow(unused_must_use)] // 忽略警告
fn ex_expression() {
    let x = 5u32;

    // 表达式 - 自动赋值
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x // 表达式, 自动返回值
    };

    // 语句 - 不赋值
    let z = {
        2 * x; // 语句: 无返回值, 默认返回 ()
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
