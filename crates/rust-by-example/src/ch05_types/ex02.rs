// 字面量 - 后缀标识
// 内存占用查看
#[test]
fn ex02_literals() {
    //
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    //
    let i = 1;
    let f = 1.0;

    // 内存占用: // 数据类型
    println!("size of `x` in bytes: {}, type={}", std::mem::size_of_val(&x), type_of(x));
    println!("size of `y` in bytes: {}, type={}", std::mem::size_of_val(&y), type_of(y));
    println!("size of `z` in bytes: {}, type={}", std::mem::size_of_val(&z), type_of(z));
    println!("size of `i` in bytes: {}, type={}", std::mem::size_of_val(&i), type_of(i));
    println!("size of `f` in bytes: {}, type={}", std::mem::size_of_val(&f), type_of(f));
}

// 数据类型判断:
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
