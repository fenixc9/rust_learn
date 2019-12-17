#![feature(async_await, async_closure)]

use std::thread;
use std::time::Duration;
use futures::Future;
use futures::executor;
use std::thread::sleep;
use std::sync::mpsc;

fn main() {
    let f11 = f1();
    let f21 = f2();
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` is similar to `Iterator::next`, but returns a
    // type that implements `Future<Output = Option<T>>`.
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn f1() {
    println!("work1 start...");
    sleep(Duration::from_secs(2));
    println!("work1 finished...");
}

async fn f2() {
    println!("work2 start...");
    sleep(Duration::from_secs(2));
    println!("work2 finished...");
}