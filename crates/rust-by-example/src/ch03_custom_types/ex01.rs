#[test]
fn ex01_structs() {
	#[derive(Debug)]
	struct Person {
		name: String,
		age: u8,
	}

	// 单元结构体:
	struct Unit;

	// 元组结构体:
	struct Pair(i32, f32);

	struct Point {
		x: f32,
		y: f32,
	}

	// 组合式:
	#[allow(dead_code)]
	struct Rectangle {
		top_left: Point, // 组合式结构体
		bottom_right: Point,
	}

	////////////////////////////////////////////////////////////////////////////////

	let name = String::from("Peter");
	let age = 27;
	let peter = Person { name, age };
	println!("{:?} => {}, {}", peter, peter.name, peter.age);

	////////////////////////////////////////////////////////////////////////////////

	//
	let point = Point { x: 10.3, y: 0.4 };
	println!("point coordinates: ({}, {})", point.x, point.y);

	// 解包:
	let bottom_right = Point { x: 5.2, ..point };
	println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

	////////////////////////////////////////////////////////////////////////////////

	let _unit = Unit;
	let pair = Pair(1, 0.1);
	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	let Pair(integer, decimal) = pair;
	println!("pair contains {:?} and {:?}", integer, decimal);
}
