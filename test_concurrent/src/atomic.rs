use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub fn atomic_test() {
    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    for x in 0..10 {
        let share_var = var.clone();

        // 创建一个新线程
        let new_thread = thread::spawn(move || {
            println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
            // 修改值
            share_var.store(x, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_secs(1));
    // 等待新建线程先执行
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}
