#[test]
fn ex02_scope() {
    let long_live_binding = 1;

    // 局部作用域:
    {
        let short_live_binding = 2;
        println!("inner short_live_binding: {}", short_live_binding);

        // 变量遮蔽:
        let long_live_binding = 5_f32;
        println!("inner long_live_binding: {}", long_live_binding);
    }

    ////////////////////////////////////////////////////////////////////////////////

    // error[E0384]: cannot find value `short_live_binding` in this scope
    // println!("outer short: {}", short_live_binding);

    println!("outer long_live_binding: {}", long_live_binding);

    ////////////////////////////////////////////////////////////////////////////////

    // 变量遮蔽:
    let long_live_binding = 'a';
    println!("outer long_live_binding: {}", long_live_binding);
}
