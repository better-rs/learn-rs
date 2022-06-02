use std::fmt::Debug;

#[test]
#[allow(dead_code)]
fn ex01_generics() {
	//
	fn foo<T: Debug>(arg: T) {
		println!("{:?}", arg);
	}

	////////////////////////////////////////////////////////////////////////////////
	#[derive(Debug)]
	struct A;

	#[derive(Debug)]
	struct Single(A);

	#[derive(Debug)]
	struct SingleGen<T>(T);

	////////////////////////////////////////////////////////////////////////////////

	let _s = Single(A);
	let _char = SingleGen('a');
	let _t = SingleGen(A);
	let _i32 = SingleGen(32);
	let _char = SingleGen('a');

	println!("{:?}", _s);
	println!("{:?}", _char);
	println!("{:?}", _t);
	println!("{:?}", _i32);
	println!("{:?}", _char);
}

#[test]
fn ex01_01_gen_fn() {
	struct A;
	struct S(A);
	struct SGen<T>(T);

	fn reg_fn(_s: S) {
		println!("reg_fn is called");
	}

	fn gen_spec_t(_s: SGen<A>) {
		println!("gen_spec_t is called");
	}
	fn gen_spec_i32(_s: SGen<i32>) {
		println!("gen_spec_i32 is called");
	}
	fn generic<T>(_s: SGen<T>) {
		println!("generic is called");
	}

	////////////////////////////////////////////////////////////////////////////////

	reg_fn(S(A));
	gen_spec_t(SGen(A));
	gen_spec_i32(SGen(32));
	generic::<char>(SGen('a'));
	generic(SGen(A));
}
