#[test]
fn ex01() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = (); // 空元组

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    // 隐藏警告:
    #[allow(unused_variables)]
    let noisy_unused_variable = 2u32;
}
