use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[allow(unused)]
fn main() {
    /*
    todo x: 并发代码 - 同步阻塞
        1. 实现等待子线程先 run, 然后 main 线程接收信号, 再继续执行.
        2. 特别说明:
            - Arc / Mutex / Condvar 是并发安全的, 经常组合使用.(实现外部不可变, 内部可变)
            - Arc 是外部不可变的, 需要搭配 Mutex/Condvar.
            - Mutex 是内部可变的,
        3. 注意作用域, 生存周期.

    */

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
