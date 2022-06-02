mod ch01_hello;
mod ch02_primitives; // 必须导入, 否则内部的 #[test] 标记, 无法识别
mod ch03_custom_types;
mod ch04_variable_bindings;
mod ch05_types;
mod ch06_conversion;
mod ch07_expressions;
mod ch08_flow_control;
mod ch09_fn;
mod ch13_attribute;
mod ch14_generics;
mod ch15_scope;
mod ch16_trait;
mod ch17_macros;
mod ch18_error;
mod ch19_std;
mod ch20_std_misc;
mod ch21_testing;
mod ch22_unsafe;

//
// extends:
//
pub mod x; // TODO X: 添加的一些扩展辅助函数 // 对应 ch05/ex03.rs

// use x::types::*; // TODO X: 导包方式 1, 项目内导包

/*

参考:
	https://rustwiki.org/zh-CN/rust-by-example/index.html

说明:
	1. 根据例子学习Rust语言
	2. 借助 IDE 可以快速执行 test 方法的技巧, 快速学习.

 */

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
