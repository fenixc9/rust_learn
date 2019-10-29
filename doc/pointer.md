# 智能指针
几个常见的指针为：
* Rc
* Arc
* Box
* Ref
* Refcell

### Rc
引用计数功能的指针，当引用计数归零的时候自动回收.
1. `clone`时strong_count+1;
```rust
#[test]
fn t1() {
    let rc = Rc::new(32);
    assert_eq!(1, Rc::strong_count(&rc));
    let rc1 = rc.clone();
    assert_eq!(2, Rc::strong_count(&rc));
}
```
调用strong_count计算rc需要传入rc指针的引用，不能传入所有权。

2. rc指针是一个指针，不影响对象的方法调用形式，也就是不需要解引用的操作。直接就可以操作对象的方法。
```rust
#[test]
fn t2() {
    let rc = Rc::new("this is a str");
    assert_eq!(rc.len(), 13);
}
```

3. `drop`是显式回收
```rust
#[test]
fn t3() {
    let rc = Rc::new("this is a str");
    let rc1 = rc.clone();
    let rc2 = rc.clone();
    let rc3 = rc.clone();

    println!("{}", Rc::strong_count(&rc));
    drop(rc3);
    println!("{}", Rc::strong_count(&rc));
}
```
`drop` 后`strong_count`就会减1

### Arc
是Rc的跨线程版本


