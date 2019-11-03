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

如果连续调用`borrow_mut`生成可变引用就会在运行时出现panic
```rust
pub fn test_refcell_multi_borrow_mut() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow_mut();
    let s2 = shard_vec.borrow_mut();
    // panicked at 'already borrowed: BorrowMutError', src\libcore\result.rs:1084:
}
```

但是如果一次调用生成`borrow_mut`后立刻结束它的声明周期是没问题的。
```rust
pub fn test_refcell_multi_borrow_mut_1() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    shard_vec.borrow_mut().push(4);
    shard_vec.borrow_mut().push(5);
    // RefCell { value: [1, 2, 3, 4, 5] }
}
```

`borrow`同时存在多个则是没问题的
```rust
pub fn test_refcell_multi_borrow_() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow();
    let s2 = shard_vec.borrow();
    println!("{:?}",shard_vec);
    // RefCell { value: [1, 2, 3] }
}
```

同时存在`borrow`和`borrow_mut`也是不行的
```rust
pub fn test_refcell_borrow_and_borrow_mut() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow();
    let s2 = shard_vec.borrow_mut();
    println!("{:?}",shard_vec);
    // panicked
}
```

但是我们为了避免运行时异常，可以用`try_borrow`和`try_borrow_mut`
```rust
pub fn test_refcell_try_borrow() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let x = match shard_vec.try_borrow_mut() {
        Ok(v) => { v }
        _ => {
            panic!("LOG: shard_vec has been borrowed");
        }
    };
    let x1 = match shard_vec.try_borrow_mut() {
        Ok(v) => { v }
        _ => {
            panic!("LOG: shard_vec has been borrowed");
        }
    };
}
```
这里也是直接抛出panic
### Rc和Refcell
Rc和Arc和Refcell，cell这些具有内部可变性的指针常常一起用，比如`Rc<Refcell<T>>`这样

一个例子
```rust
pub fn test_rc_refcell_1() {
    let rc1 = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("1strong count:{}", Rc::strong_count(&rc1));

    modify1(rc1.clone());
    println!("2strong count:{}", Rc::strong_count(&rc1));

    modify2(rc1.clone());
    println!("3strong count:{}", Rc::strong_count(&rc1));
    println!("{:?}", rc1);
    // 1strong count:1
    //4strong count:2
    //2strong count:1
    //5strong count:2
    //3strong count:1
}

fn modify1(i: Rc<RefCell<Vec<i32>>>) {
    println!("4strong count:{}", Rc::strong_count(&i));
    i.borrow_mut().push(4);
}

fn modify2(i: Rc<RefCell<Vec<i32>>>) {
    println!("5strong count:{}", Rc::strong_count(&i));
    i.borrow_mut().push(4);
}
```
在这段示例中，`test_rc_refcell_1`中观察到的Rc strongcount始终是1，
而传入modify函数的Rc会随着modify执行的结束自动回收，所以最后当这个用例fn执行完毕时，
所有Rc指针都会被自动回收。Rc归零，资源回收。