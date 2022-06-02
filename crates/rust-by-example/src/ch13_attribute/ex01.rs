#[test]
fn ex01_unused() {
	fn used_function() {
		println!("I'm used!");
	}

	#[allow(dead_code)] // 禁用 `dead_code` lint
	fn unused_function() {
		println!("I'm unused!");
	}

	// call:
	used_function();
}
