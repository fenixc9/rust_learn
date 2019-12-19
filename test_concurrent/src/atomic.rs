use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub fn atomic_test() {
    const COUNT: i32 = 2000;
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        for x in 0..COUNT {
            spinlock_clone.fetch_add(1, Ordering::SeqCst);
        }
    });

    let spinlock_clone1 = spinlock.clone();
    let thread1 = thread::spawn(move || {
        for x in 0..COUNT {
            spinlock_clone1.fetch_add(1, Ordering::SeqCst);
        }
    });

    let spinlock_clone2 = spinlock.clone();
    let thread2 = thread::spawn(move || {
        for x in 0..COUNT {
            spinlock_clone2.fetch_add(1, Ordering::SeqCst);
        }
    });
    thread.join();
    thread1.join();
    thread2.join();
    println!("result:{:?}", spinlock);
    // 6001
}

