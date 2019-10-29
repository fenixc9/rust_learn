use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[test]
fn t1() {
    static mut VAR: i32 = 1;
    for _ in 0..10 {
        thread::spawn(move || {
            for _ in 0..10 {
                // 完全不同步
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
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn t3() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
