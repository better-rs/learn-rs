use std::{
	sync::{Arc, Condvar, Mutex},
	thread,
	time::Duration,
};

fn main() {
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = Arc::clone(&pair);

	let t = thread::spawn(move || {
		let (lock, cvar) = &*pair2;

		/// todo x: 官方标准用法的巧妙点:
		///     1. 这里只是指针方式, 改变 lock 内部元素的值, 因为没有构造新变量.
		/// 所以不会引入新的生命周期.     2. 这种写法, 就不会因引入新变量(锁赋值), 导致 lock
		/// 阻塞整块作用域.
		*lock.lock().unwrap() = true;
		println!("\tSub >>I'm a happy worker!");

		// 通知主线程
		cvar.notify_one();
		//mem::drop(started);
		//drop(started);

		for i in 0..5 {
			thread::sleep(Duration::from_millis(500));
			println!("\tSub >> I'm a happy worker! {}", i);
		}
	});

	println!("Main >> waiting worker to start!");
	// 等待工作线程的通知
	let (lock, cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	while !*started {
		started = cvar.wait(started).unwrap();
	}

	t.join().unwrap();
	println!("Main >> run done!");
}
