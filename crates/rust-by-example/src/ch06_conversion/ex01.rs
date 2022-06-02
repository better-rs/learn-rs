/*

说明:
	1. From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。
	2. Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。

*/

#[test]
#[allow(dead_code)]
fn ex01_from_into() {
	// From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。
	// Into trait 就是把 From trait 倒过来而已。
	// 也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。

	let my_str = "hello";
	let my_string = String::from(my_str);
	println!("my string: {}", my_string);

	#[derive(Debug)]
	struct Number {
		value: i32,
	}

	impl From<i32> for Number {
		// 类型转换
		fn from(item: i32) -> Self {
			Number { value: item }
		}
	}

	////////////////////////////////////////////////////////////////////////////////

	// 初始化: 类型转换
	let num = Number::from(30);
	println!("My number is {:?}", num);

	////////////////////////////////////////////////////////////////////////////////

	let x = 5;
	// Into trait 就是把 From trait 倒过来而已。
	// 也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
	let num: Number = x.into();
	println!("My number is {:?}", num);
}
