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

    ///
    /// todo x: 发送者: 发送信号
    ///
    let sender = move || {
        println!("\t[t] sender entering...");
        let (lock, cvar) = &*pair2;
        {
            let mut started = lock.lock().unwrap();
            *started = true;
        }

        cvar.notify_all();

        for i in 0..4 {
            println!("\t[t] sender {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        println!("\t[t] sender exiting...");
    };

    ///
    /// todo x: 接收者: 等待信号
    ///
    let receiver = move || {
        println!("[t] receiver entering...");
        let (lock, cvar) = &*pair;

        {
            let mut started = lock.lock().unwrap();
            println!("[t] receiver status: {}", started);
            while !*started {
                started = cvar.wait(started).unwrap();
                println!("[t] receiver check status: {}", started);
            }
        }

        println!("[t] receiver exiting...");
    };

    let t1 = thread::spawn(sender);
    let t2 = thread::spawn(receiver);
    t1.join().unwrap();
    t2.join().unwrap();

    //
    //
    //
    println!("Main >> run done!");

    thread::sleep(Duration::from_secs(5));
}
