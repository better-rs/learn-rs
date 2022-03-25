use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let t = thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        //let mut started = lock.lock().unwrap();
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
