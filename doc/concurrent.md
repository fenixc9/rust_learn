# 并发
并发跟其他语言区别看似很大，但是底层操作系统和硬件的实现其实不可能有太大区别。

所以Rust的并发控制本质上还是和其他语言类似的。

底层api控制就是基本的mutex，atomic。

应用层就是和golang类似的channel，区别在于Rust的channel的生产者分为同步发送和异步发送。

Rust的思想是"共享不可变，可变不共享"

### channel
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

```
`mpsc::channel`这个形式构建出来的是异步Sender

可以看到，上例在消费者阻塞的情况下，生产者，仍然能立刻将所有信息立刻发送完毕，打印出send finish

```rust
#[test]
fn t3() {
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
```
`mpsc::sync_channel`这个形式构建出来的是同步Sender

可以看到，在消费者被阻塞的情况下，生产者也被阻塞了。

##### 多生产者，单消费者
mpsc channel的应用场景通常是clone多出个，构建多个生产者，分散到多个线程中，然后由一个消费者消费，
跟他的名称相符（mpsc:multi producer single consumer）
```rust
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
    //receiver:1
    //receiver:2
    //...
    //receiver:993
    //receiver:994
    //receiver:995
    //receiver:996
    //receiver:997
    //receiver:998
    //receiver:999
    //receiver:1000
}

```
可以看到上面的示例代码正确的输出了生产者发送的消息数，而消息是由10个线程分别发送的。

### thread
Rust线程就是Os线程
创建线程通常调用spawn传入一个lambda表达式。
```rust
pub fn create_thread() {
    let handle = thread::spawn(|| {
        for x in 0..10 {
            println!("{}", x);
            thread::sleep(Duration::from_secs(1))
        }
    });
    handle.join();
}
```
闭包外传入要加move语义
```rust
pub fn create_thread_no_move() {
    let a = 10;
    let handle = thread::spawn(|| {
        println!("{}",a);
    });
    handle.join();
}
```
按照move语义，被移动到闭包里就不能在调用了
```rust
pub fn create_thread_move() {
    println!("create_thread_move");
    let a = "this is str".to_string();
    let handle = thread::spawn(move || {
        println!("spawn {}",a);
    });
    thread::sleep(Duration::from_secs(1));
    println!("create_thread_move {}",a);
    handle.join();
}
```
上面的代码会报错`value borrowed here after move`.
##### thread park
停止当前线程
```rust
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
```

### Arc指针
用于跨线程的Rc操作。通常可以用于跨线程共享数据

```rust
pub fn mutex_test() {
    const COUNT: i32 = 1000;
    let arc = Arc::new(Mutex::new(1));
    let arcClone = arc.clone();
    let handle = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone.lock().unwrap();
            *guard1 += 1;
        }
    });

    let arcClone1 = arc.clone();
    let handle1 = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone1.lock().unwrap();
            *guard1 += 1;
        }
    });

    let arcClone2 = arc.clone();
    let handle2 = thread::spawn(move || {
        for i in 0..COUNT {
            let mut guard1 = arcClone2.lock().unwrap();
            *guard1 += 1;
        }
    });
    handle.join();
    handle1.join();
    handle2.join();
    let guard = arc.lock().unwrap();
    println!("result:{}", guard);
}
```

### 重排序
Rust的原子操作需要指定重排序，
##### Relaxed ordering 
Relaxed ordering 只能保证原子操作，在同一个线程里面，对同一个变量的操作会满足 happens-before 关系