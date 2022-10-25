use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

#[allow(unused)]
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    let t = thread::spawn(move || {
        let (l, cv) = &*pair2;

        // todo x: 加锁范围
        let mut started = l.lock().unwrap();
        *started = true;

        // todo x: 注意释放时机
        // We notify the condvar that the value has changed.
        cv.notify_one();

        ///
        /// todo x: 注意加锁范围 和 释放时机
        println!("\tsub: task finished")
    });

    t.join().ok().unwrap();

    // Wait for the thread to start up.
    let (l, cv) = &*pair;

    // todo x: 等待获取锁
    let mut started = l.lock().unwrap();
    while !*started {
        // todo x: 阻塞等待
        started = cv.wait(started).unwrap();
    }
    println!("main: finished waiting!");
}
