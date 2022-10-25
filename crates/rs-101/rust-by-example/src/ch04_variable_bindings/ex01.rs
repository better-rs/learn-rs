#[test]
fn ex01() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = (); // 空元组

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // _开头, 不会警告
    let _unused_variable = 3u32;

    // 隐藏警告:
    #[allow(unused_variables)]
    let noisy_unused_variable = 2u32;
}

#[test]
fn ex01_mut() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // error[E0384]: cannot assign to `_immutable_binding` because it is immutable
    // _immutable_binding += 1;
}
