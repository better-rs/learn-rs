// 数据类型判断:
pub fn type_of<T>(_: T) -> &'static str {
	// 基于标准库扩展
	std::any::type_name::<T>()
}
