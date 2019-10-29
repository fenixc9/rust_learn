# 并发
并发跟其他语言区别看似很大，但是底层操作系统和硬件的实现其实不可能有太大区别。

所以Rust的并发控制本质上还是和其他语言类似的。

底层api控制就是基本的mutex，原子类。

应用层就是和golang类似的channel，channel的生产者分为同步发送和异步发送。

## channel
```rust
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
```
`mpsc::channel`这个形式构建出来的是异步Sender
`mpsc::sync_channel`这个形式构建出来的是同步Sender
