use std::mem;

// 数组和切片 // 函数传参: 切片参数
#[test]
fn ex03_array() {
	// 入参: 引用类型
	fn analyze_slice(slice: &[i32]) {
		println!("\tfirst element of the slice: {}", slice[0]);
		println!("\tthe slice has {} elements", slice.len());
	}

	let xs: [i32; 5] = [1, 2, 3, 4, 5];
	let ys: [i32; 500] = [0; 500];

	println!("first element of the array: {}", xs[0]);
	println!("second element of the array: {}", xs[1]);
	println!("array size: {}", xs.len());
	// mem size:
	println!("array occupies {} bytes", mem::size_of_val(&xs));

	////////////////////////////////////////////////////////////////////////////////

	println!("borrow the whole array as a slice");
	// 完整数组: 引用类型
	analyze_slice(&xs);

	println!("borrow a section of the array as a slice");
	// 部分数组
	analyze_slice(&ys[1..4]);
	// println!("{}", xs[5]); // 下标越界
}
