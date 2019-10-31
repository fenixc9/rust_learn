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

### Cell 指针
内部可变性，参考如下代码
```rust
pub fn cell_use() {
    let cell = Cell::new(0);
    cell.set(1);
    let i = cell.get();
    assert_eq!(i,1)
}
```
可见，这里cell并不是mut变量，但是依然可以靠set get函数修改

### Refcell指针
`Cell`指针只能整体替换Cell包裹的数据，但是`Refcell`指针却可以获得一个直接修改内部数据的指针。

本质上Rust是在编译器检查借用的，而Refell是在运行时检查借用，也是完全相同的原则:

**可以多个读，不能一个读和一个写，不能多个写**

用户通常使用`borrow`和`borrow_mut`来分别获取可变指针和不可变指针。

**通常需要和Rc和Arc配合使用**。

如果是整体存入就用`Cell`，如果是部分修改就用`Refcell`
```rust
pub fn test_refcell() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = &shard_vec;
    let s2 = &s1;
    s1.borrow_mut().push(4);
    println!("{:?}",s1.borrow());

    s2.borrow_mut().push(5);
    println!("{:?}",s2.borrow());
    //[1, 2, 3, 4]
    //[1, 2, 3, 4, 5]
}
```