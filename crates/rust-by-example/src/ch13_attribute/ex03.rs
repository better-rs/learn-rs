#[test]
fn ex03_cfg() {
	/*
	TODO X:
		条件编译可能通过两种不同的操作符实现：
			cfg 属性：在属性位置中使用 #[cfg(...)]
			cfg! 宏：在布尔表达式中使用 cfg!(...)
	*/

	#[cfg(target_os = "linux")]
	fn are_you_on_linux() {
		println!("You are on Linux!");
	}

	#[cfg(not(target_os = "linux"))]
	fn are_you_on_linux() {
		println!("You are `not` on Linux!");
	}

	////////////////////////////////////////////////////////////////////////////////`

	// call:
	are_you_on_linux();
	println!("Are you sure?");
	if cfg!(target_os = "linux") {
		println!("Yes, It's definitely Linux!");
	} else {
		println!("No, It's definitely `not` Linux!");
	}
}

#[test]
fn ex03_01_custom() {
	#[cfg(some_condition)]
	fn conditional_function() {
		println!("condition met!");
	}

	// conditional_function();
}

/*

TODO X:
	$ rustc custom.rs && ./custom
	No such file or directory (os error 2)
	$ rustc --cfg some_condition custom.rs && ./custom
	condition met!

*/
