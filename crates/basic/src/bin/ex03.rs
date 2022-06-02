use std::thread;

fn main() {
	//! 异步管道: 1个发送者 vs 1个接收者

	let (tx, rx) = std::sync::mpsc::channel();

	thread::spawn(move || {
		for i in 0..5 {
			thread::sleep(std::time::Duration::from_millis(500));

			tx.send(i).unwrap();
			let thread_id = thread::current().id();
			println!("\tsender {:02?} sent {}", thread_id, i);
		}
	});

	// receiver:
	while let Ok(received) = rx.recv() {
		let thread_id = thread::current().id();
		println!("main >> receiver {:02?} received {}", thread_id, received);
	}
}
