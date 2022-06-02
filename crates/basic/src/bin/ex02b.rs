use std::{
	self,
	sync::{Arc, Condvar, Mutex},
	thread,
	time::Duration,
};
//use std::time::Duration;

#[allow(unused)]
fn main() {
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = Arc::clone(&pair);

	let t = thread::spawn(move || {
		let (l, cv) = &*pair2;

		let mut ok = l.lock().unwrap();
		*ok = true;

		println!("I'm a happy worker!");
		// 通知主线程
		cv.notify_one();

		///
		/// todo x: 无效
		// drop(ok); // todo x: 无效
		std::mem::drop(ok); // todo x: 无效

		///
		let mut i = 0;
		while i < 3 {
			i += 1;
			thread::sleep(Duration::from_secs(1));
			println!("\tSub >> sleep {}", i);
		}
	});

	// todo x: join 位置:
	// t.join().ok().unwrap();

	println!("main: waiting worker to start!");

	// 等待工作线程的通知
	let (l, cv) = &*pair;

	// 锁获取: 阻塞
	let mut ok = l.lock().unwrap();

	while !*ok {
		println!("main: while before ... {}", ok);
		// 阻塞等:
		ok = cv.wait(ok).unwrap();
		println!("main: while after... {}", *ok);
	}

	t.join().ok().unwrap();

	println!("main: run done!");
}
