use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[allow(unused)]
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let t = thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        /// todo x: 释放锁
        {
            let mut ok = lock.lock().unwrap();
            *ok = true;

            eprintln!("\tSub >> I'm a happy worker!");
        }

        /// todo x: 局部释放
        {
            // 通知主线程
            cvar.notify_one();
        }

        ///
        ///
        let mut i = 0;
        while i < 3 {
            i += 1;
            thread::sleep(Duration::from_secs(2));
            println!("\tSub >> sleep {}", i);
        }
    });

    println!("Main >> waiting worker to start!");
    // 等待工作线程的通知
    let (l, cv) = &*pair;

    ///
    /// todo x: 阻塞等待 获取锁
    ///
    let mut ok = l.lock().unwrap();
    while !*ok {
        println!("Main >> waiting before: {}", ok);

        //
        // todo x: 阻塞等待 通知
        //
        ok = cv.wait(ok).unwrap();
        println!("Main >> waiting after: {}", ok);
    }

    //
    //
    //
    println!("Main >> Worker started!");
    t.join().unwrap();
}
