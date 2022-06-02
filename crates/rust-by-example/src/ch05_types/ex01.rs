/*

使用 as 关键字进行显 式类型转换（casting）。

*/
#[test]
// #[allow(overflowing_literals)]
fn ex01_cast() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal ;
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!("1000 as a u16 is: {}", 1000 as u16);

    // error: literal out of range for `u8`
    // println!("1000 as a u8 is: {}", 1000 as u8); //  type `u8` whose range is `0..=255`

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);
    println!("1000 mod 256 is: {}", 1000 % 256);

    //
    println!("128 as a i16 is: {}", 128 as i16);
    // println!("128 as a i8 is: {}", 128 as i8); // type `i8` whose range is `-128..=127`

    // rustc 1.58.1 (db9d1b20b 2022-01-20) // 如下报错:
    // println!("1000 as a u8 is: {}", 1000 as u8);
    // println!("232 as a i8 is: {}", 232 as i8);

    // 转换:
    println!("300.0 as u8 is {}", 300.0_f32 as u8); // 255
    println!("-100.0 as u8 is {}", (-100.0_f32) as u8); // =0
    println!("nan as u8 is {}", std::f32::NAN as u8); // 0

    // 溢出:
    unsafe {
        println!("unsafe > 300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("unsafe > -100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>()); // =0
        println!("unsafe > nan as u8 is {}", std::f32::NAN.to_int_unchecked::<u8>()); // 0
    }
}
