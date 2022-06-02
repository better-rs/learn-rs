#[test]
fn ex08_assoc_items() {
	/*
	TODO X:
		“关联项”（associated item）指与多种类型的项有关的一组规则。
		它是 trait 泛型的扩展，允许在 trait 内部定义新的项。
	*/

	struct Container(i32, i32);

	trait Contains<A, B> {
		fn contains(&self, _: &A, _: &B) -> bool;
		fn first(&self) -> i32;
		fn last(&self) -> i32;
	}

	impl Contains<i32, i32> for Container {
		fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
			(&self.0 == number_1) && (&self.1 == number_2)
		}
		fn first(&self) -> i32 {
			self.0
		}
		fn last(&self) -> i32 {
			self.1
		}
	}
	////////////////////////////////////////////////////////////////////////////////

	fn difference<A, B, C>(container: &C) -> i32
	where
		C: Contains<A, B>,
	{
		container.last() - container.first()
	}

	////////////////////////////////////////////////////////////////////////////////

	let number_1 = 3;
	let number_2 = 10;

	let container = Container(number_1, number_2);

	println!(
		"Does container contain {} and {}: {}",
		number_1,
		number_2,
		container.contains(&number_1, &number_2)
	);

	println!("First number: {}", container.first());
	println!("Last number: {}", container.last());

	println!("Difference: {}", difference(&container));
}

#[test]
fn ex08_02_types() {
	/*
	TODO X:
		关联类型: 通过把容器内部的类型放到 trait 中作为输出类型，使用 “关联类型” 增加了代码 的可读性

		trait Contains {
			type A;
			type B;

			fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
		}

		// TODO X: 简写
		fn diff<C>(container: &C) -> i32 {
			container.last() - container.first()
		}
	*/

	////////////////////////////////////////////////////////////////////////////////

	struct Container(i32, i32);

	trait Contains {
		type A;
		type B;

		fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
		fn first(&self) -> i32;
		fn last(&self) -> i32;
	}

	impl Contains for Container {
		type A = i32;
		type B = i32;

		fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
			(&self.0 == number_1) && (&self.1 == number_2)
		}

		fn first(&self) -> i32 {
			self.0
		}

		fn last(&self) -> i32 {
			self.1
		}
	}

	////////////////////////////////////////////////////////////////////////////////

	fn diff<C: Contains>(container: &C) -> i32 {
		container.last() - container.first()
	}

	////////////////////////////////////////////////////////////////////////////////

	let number_1 = 3;
	let number_2 = 10;
	let container = Container(number_1, number_2);
	println!(
		"Does container contain {} and {}: {}",
		number_1,
		number_2,
		container.contains(&number_1, &number_2)
	);
	println!("First number: {}", container.first());
	println!("Last number: {}", container.last());
	println!("Difference: {}", diff(&container));
}
