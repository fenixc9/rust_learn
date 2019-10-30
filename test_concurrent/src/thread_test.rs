use std::thread;
use std::sync::Arc;
use std::time::Duration;

pub fn create_thread() {
    let handle = thread::spawn(|| {
        for x in 0..10 {
            println!("{}", x);
            thread::sleep(Duration::from_secs(1))
        }
    });
    handle.join();
}

pub fn park_thread() {
    let handle = thread::spawn(|| {
        for x in 0..10 {
            if x > 4 {
                thread::park()
            }
            println!("{}", x);
            thread::sleep(Duration::from_secs(1))
        }
    });
    handle.join();
}

pub fn create_thread_move() {
//    println!("create_thread_move");
//    let a = "this is str".to_string();
//    let handle = thread::spawn(move || {
//        println!("spawn {}",a);
//    });
//    thread::sleep(Duration::from_secs(1));
//    println!("create_thread_move {}",a);
//    handle.join();
}

pub fn create_thread_no_move() {
//    let a = 10;
//    let handle = thread::spawn(|| {
//        println!("{}",a);
//    });
//    handle.join();
}


