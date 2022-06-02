#[test]
fn ex03_declare() {
	//
	let a_binding;

	// 局部作用域:
	{
		let x = 2;

		a_binding = x * x;
	}

	println!("a binding: {}", a_binding);

	let another_binding;
	// 禁止使用未初始化的变量:
	// println!("another binding: {}", another_binding);

	another_binding = 1;
	println!("another binding: {}", another_binding);
}
