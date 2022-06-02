#[test]
fn ex() {
	let logical: bool = true; // 标记类型
	let a_float: f64 = 1.0; // 主动标记类型
	let an_integer = 5i32; // 后缀类型
	println!("{} {} {}", logical, a_float, an_integer);

	// 自动推导类型
	let default_float = 3.0; // f64
	let default_integer = 7; // i32
	println!("{} {}", default_float, default_integer);

	// 上下文推导类型
	let mut inferred_type = 12;
	println!("before: {}", inferred_type);
	inferred_type = 4294967296i64; // i64
	println!("after: {}", inferred_type);

	let mut mutable = 12; // mutable
	println!("before: {}", mutable);
	mutable = 21;
	// mutable = true; // error
	println!("after: {}", mutable);

	// 变量遮蔽: 变量名相同，但类型不同
	let mutable = true; // mutable
	println!("shadow: {}", mutable);
}

#[test]
fn ex01_literals() {
	println!("1 + 2 = {}", 1u32 + 2);
	println!("1 - 2 = {}", 1i32 - 2);

	// 布尔求值
	println!("true AND false is {}", true && false);
	println!("true OR false is {}", true || false);
	println!("NOT true is {}", !true);

	// 位运算:
	println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
	println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
	println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
	// 移位操作:
	println!("1 << 5 is {}", 1u32 << 5);
	println!("1 << 10 is {}", 1u32 << 10);
	println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

	// 下划线: 可读性
	println!("One million is written as {}", 1_000_000u32);
}
