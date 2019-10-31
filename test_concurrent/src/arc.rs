use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn arc_cross_thread() {
    let arc = Arc::new(String::from("aabbcc"));

    for index in 0..10 {
        let a = arc.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(300));
            println!("{},{:?}", Arc::strong_count(&a), thread::current());
            drop(a);
        });
    }
    thread::sleep(Duration::from_secs(3));
    let i = Arc::strong_count(&arc);
    println!("{}", i);
}

/// 一写多读
pub fn arc_one_write_multi_read() {
    let mut arc = Arc::new(RefCell::new(String::from("abc")));

    for index in 0..10 {
        let a = arc.clone();
        thread::spawn(move || {
            for x in 0..100 {
                if index == 1 {
                    arc.into_inner().push_str("3");
                    println!("{},{:?}",arc.into_inner(), thread::current());
                }
                println!("{:?}", thread::current());
            }
        });
    }
    thread::sleep(Duration::from_secs(3));
    println!("{}", arc);
}