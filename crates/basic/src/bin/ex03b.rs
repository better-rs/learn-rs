use std::thread;

#[allow(unused)]
fn main() {
    //! 异步管道: 多个发送者 vs 1个接收者

    let (tx, rx) = std::sync::mpsc::channel();

    /// multi sender:
    for i in 0..10 {
        let tx = tx.clone();

        // 并发:
        thread::spawn(move || {
            let msg = format!("Hello {}", i);
            let thread_id = thread::current().id();

            println!("\tsender {:02?} sent {}", thread_id, msg);
            tx.send(msg).unwrap();
        });
    }

    drop(tx);

    // receiver:
    while let Ok(received) = rx.recv() {
        let thread_id = thread::current().id();
        println!("main >> receiver {:02?} received {}", thread_id, received);
    }
}
