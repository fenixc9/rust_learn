use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_sync_channel() {
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
        }
        println!("send finish");
    });

    for received in rx {
        thread::sleep(Duration::from_secs(1));
        println!("Got: {}", received);
    }
    //send finish
    //Got: hi
    //Got: from
    //Got: the
    //Got: thread
}

pub fn run_async_channel() {
    let (tx, rx) = mpsc::sync_channel(0);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
        println!("send finish");
    });

    for received in rx {
        println!("Got: {}", received);
        thread::sleep(Duration::from_secs(1));
    }
    //Got: hi
    //Got: from
    //Got: the
    //Got: thread
    //send finish
}

pub fn multi_sender_one_receiver() {
    let (send, receiver) = mpsc::channel();
    for i in 0..10 {
        let sender = send.clone();
        let handle = thread::spawn(move || {
            for y in 0..100 {
                sender.send(y).unwrap();
            }
        });
    }
    let mut counter = 0;
    for x in receiver {
        counter += 1;
        println!("receiver:{}", counter);
    }
}