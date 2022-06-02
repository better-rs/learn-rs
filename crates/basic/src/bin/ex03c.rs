use std::thread;

#[allow(unused)]
fn main() {
	//! 同步管道(sync_channel):
	//!     - 固定缓冲区大小
	//!     - 缓冲区满时, 发送者阻塞

	// 限定缓冲区大小:
	let (tx, rx) = std::sync::mpsc::sync_channel(1);

	tx.send(1).unwrap();
	println!("send 1");

	thread::spawn(move || {
		for i in 0..10 {
			let thread_id = thread::current().id();
			println!("\t{:02?} send: {}", thread_id, i);
			thread::sleep(std::time::Duration::from_millis(1500));
			tx.send(i).unwrap();
		}
	});

	// receiver:
	while let Ok(received) = rx.recv() {
		let thread_id = thread::current().id();
		println!("main >> receiver {:02?} received {}", thread_id, received);
	}
}
