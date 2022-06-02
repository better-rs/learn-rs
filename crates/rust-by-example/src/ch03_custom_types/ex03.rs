/*

Rust 有两种常量，可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明：
	const：不可改变的值（通常使用这种）。
	static：具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）。

*/
#[test]
fn ex03_constants() {
	static LANGUAGE: &str = "Rust";
	const THRESHOLD: i32 = 10;

	// 简单比较:
	fn is_big(n: i32) -> bool {
		n > THRESHOLD
	}

	////////////////////////////////////////////////////////////////////////////////

	println!("This is {}", LANGUAGE);
	println!("The threshold is {}", THRESHOLD);
	println!("{} is {}", 42, if is_big(42) { "big" } else { "small" });
	println!("{} is {}", 2, if is_big(2) { "big" } else { "small" });

	// 报错！不能修改一个 `const` 常量。
	// THRESHOLD = 100;
}
