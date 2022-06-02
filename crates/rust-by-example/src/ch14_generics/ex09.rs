use crate::x::types::type_of;
use std::{marker::PhantomData, ops::Add};

#[test]
fn ex09_phantom() {
	/*
	TODO X:
		虚类型（phantom type）参数是一种在运行时不出现，而在（且仅在）编译时进行静态检查 的类型参数。
		可以用额外的泛型类型参数指定数据类型，这类型可以充当标记，也可以供编译时类型检查 使用。
		这些额外的参数没有存储值，也没有运行时行为。
			std::marker::PhantomData


	*/

	#[derive(PartialEq, Debug)]
	struct PhantomTuple<A, B>(A, PhantomData<B>);

	#[derive(PartialEq, Debug)]
	struct PhantomStruct<A, B> {
		first: A,
		phantom: PhantomData<B>,
	}

	////////////////////////////////////////////////////////////////////////////////

	let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
	let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

	let _struct1: PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phantom: PhantomData };
	let _struct2: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phantom: PhantomData };

	println!("tuple1: {:?}, tuple2: {:?}", _tuple1, _tuple2);
	println!("tuple1 type = {}", type_of(&_tuple1));
	println!("tuple2 type = {}", type_of(&_tuple2));

	// assert_ne!(_tuple1, _tuple2);
}

#[test]
fn ex09_testcase_units() {
	#[derive(Debug, Copy, Clone)]
	enum Inch {}

	#[derive(Debug, Copy, Clone)]
	enum Mm {}

	#[derive(Debug, Copy, Clone)]
	struct Length<Unit>(f64, PhantomData<Unit>);

	// TODO X: 加法实现:
	impl<Unit> Add for Length<Unit> {
		type Output = Length<Unit>;

		// TODO X: 加法实现:
		fn add(self, rhs: Self) -> Self::Output {
			Length(self.0 + rhs.0, PhantomData)
		}
	}

	///////////

	let one_foot: Length<Inch> = Length(12.0, PhantomData);
	let one_meter: Length<Mm> = Length(1000.0, PhantomData);

	// TODO X: 加法运算符
	let two_feet = one_foot + one_foot;
	let two_meters = one_meter + one_meter;

	println!("one foot + one foot = {:?} in", two_feet.0);
	println!("one meter + one meter = {:?} mm", two_meters.0);
}
