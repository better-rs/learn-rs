// use crate::type_of; // TODO X: 导包方式1: 对应 check lib.rs 文件中的 use type_of::*;
use crate::x::types::type_of; //  TODO X: 导包方式2: 项目内导包方式

#[test]
fn ex03_inference() {
	// 类型推导:

	let elem = 5u8;

	// 当前为空:
	let mut vec = Vec::new();
	// TODO X: 注意是编译器阶段, 就实现了推导, 此时已经知道类型
	println!("before: vec = {:?}, type={}", vec, type_of(&vec));

	// 自动推导类型:
	vec.push(elem);
	println!("after : vec = {:?}, type={}", vec, type_of(&vec));
}
