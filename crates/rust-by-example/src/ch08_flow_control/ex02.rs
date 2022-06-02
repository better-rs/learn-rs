#[test]
fn ex02_loop() {
	//
	let mut count = 0u32;

	// 死循环
	loop {
		count += 1;
		if count == 3 {
			println!("three");
			continue
		}

		//
		println!("{}", count);

		if count == 5 {
			println!("OK, that's enough");
			break
		}
	}
}

#[test]
#[allow(unreachable_code)] // 禁用 check: 无法达到的代码
#[allow(unused_labels)] // 禁用 check: 未使用的标签
fn ex02_01_nested() {
	// TODO X: 外部循环
	'outer: loop {
		println!("Entered the outer loop");

		// TODO X: 内部循环
		'inner: loop {
			println!("Entered the inner loop");

			// 死循环
			break 'outer // TODO X: 循环跳转
		}

		// TODO X: 无法执行到此处
		println!("This point will never be reached");
	}

	println!("Exited the outer loop");
}

#[test]
fn ex02_02_return() {
	let mut counter = 0;

	// TODO X: 循环跳转
	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2
		}
	};

	println!("The result is {}", result);
	assert_eq!(result, 20, "The result should be 20");
}
