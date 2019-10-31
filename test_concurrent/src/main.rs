use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod channel;
mod thread_test;
mod mutex;
mod arc;
mod atomic;
fn main() {}

#[test]
fn t1() {
    // 直接用static变量，不加任何同步措施
    static mut VAR: i32 = 1;
    for _ in 0..10 {
        thread::spawn(move || {
            for _ in 0..10 {
                unsafe {
                    VAR += 1;
                }
            }
        });
    }
    thread::sleep(Duration::from_secs(3));
    println!("{}", unsafe { VAR });
}

#[test]
fn t2() {
    channel::run_sync_channel()
}

#[test]
fn t3() {
    channel::run_async_channel()
}

#[test]
fn t4() {
    thread_test::create_thread();
}

#[test]
fn t5() {
    thread_test::park_thread();
}

#[test]
fn t6() {
    thread_test::create_thread_move();
}

#[test]
fn t7() {
    thread_test::create_thread_no_move();
}

#[test]
fn t8() {
    channel::multi_sender_one_receiver()
}

#[test]
fn t9() {
    arc::arc_cross_thread()
}

#[test]
fn t10() {
    arc::arc_one_write_multi_read()
}

#[test]
fn t11() {
    atomic::atomic_test()
}