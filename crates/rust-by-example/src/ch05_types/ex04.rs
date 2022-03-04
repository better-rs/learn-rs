use crate::x::types::type_of;

#[test]
fn ex04_alias() {
    // 别名
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)] // 命名规范警告
    type u64_t = u64; // 别名

    // 定义值: 其他类型相同
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches // 类型相同, 可以做运算
    );

    // TODO X: 类型别名, 只是别名, 并不会创建新类型
    println!(
        "type check: {}, {}, {}",
        type_of(&nanoseconds),
        type_of(&inches),
        type_of(&(nanoseconds + inches))
    );
}
