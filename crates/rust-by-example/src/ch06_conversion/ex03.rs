use std::{fmt, fmt::Formatter};

#[test]
fn ex03_string() {
	// 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。

	struct Circle {
		radius: i32,
	}

	// TODO X: 旧版本实现, 废弃
	// impl ToString for Circle {
	//     fn to_string(&self) -> String {
	//         format!("Circle of radius {:?}", self.radius)
	//     }
	// }

	impl fmt::Display for Circle {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			write!(f, "Circle of radius {}", self.radius)
		}
	}

	////////////////////////////////////////////////////////////////////////////////

	let c = Circle { radius: 6 };
	println!("{}", c.to_string());

	////////////////////////////////////////////////////////////////////////////////

	// toInt
	let parsed: i32 = "5".parse().unwrap();
	let turbo_parsed = "10".parse::<i32>().unwrap();

	let sum = parsed + turbo_parsed;
	println!("Sum: {:?}", sum);
}
