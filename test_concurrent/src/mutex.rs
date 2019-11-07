use std::thread;
use std::sync::{Arc, Mutex};
use std::borrow::{BorrowMut, Borrow};


pub fn mutex_test() {
    const COUNT: i32 = 1000;
    let arc = Arc::new(Mutex::new(1));
    let arcClone = arc.clone();
    let handle = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone.lock().unwrap();
            *guard1 += 1;
        }
    });

    let arcClone1 = arc.clone();
    let handle1 = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone1.lock().unwrap();
            *guard1 += 1;
        }
    });

    let arcClone2 = arc.clone();
    let handle2 = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone2.lock().unwrap();
            *guard1 += 1;
        }
    });
    handle.join();
    handle1.join();
    handle2.join();
    let guard = arc.lock().unwrap();
    println!("result:{}", guard);
}