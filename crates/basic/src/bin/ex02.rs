#![feature(mutex_unlock)] // todo x: fix for unstable feature with +nightly

use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

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
        let (l, cv) = &*pair2;

        println!("[t] Sub thread: entering...");

        /// todo x: 提前切换线程
        // cv.notify_one();
        let mut ok = l.lock().unwrap();
        *ok = true;

        // Mutex::unlock(ok);
        // drop(ok); // todo x: = Mutex::unlock(ok); 内部实现
        Mutex::unlock(ok);
        println!("\tSub >> I'm a happy worker!");

        /// todo x: 再次切换线程
        cv.notify_one();

        ///
        let mut i = 0;
        while i < 3 {
            i += 1;
            thread::sleep(Duration::from_secs(2));
            println!("\tSub >> sleep {}", i);
        }
    });

    // todo x: join 位置影响
    // t.join().ok();

    println!("Main >> waiting worker to start!");

    {
        // 等待工作线程的通知
        let (l, cv) = &*pair;

        ///
        /// todo x: 阻塞等待 获取锁
        let mut ok = l.lock().unwrap();
        println!("Main >> thread status: {}", ok);
        while !*ok {
            println!("Main >> waiting before: {}", ok);

            //
            // todo x: 阻塞等待 通知
            //
            ok = cv.wait(ok).unwrap();
            println!("Main >> waiting after: {}", ok);
            thread::sleep(Duration::from_secs(2));
        }
    }

    // todo x: join 位置影响
    t.join().unwrap();

    //
    //
    //
    println!("Main >> run done!");
}
