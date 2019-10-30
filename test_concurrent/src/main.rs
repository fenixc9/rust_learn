use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod channel;

fn main() {

}
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
