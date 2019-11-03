use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::borrow::Borrow;

pub fn cell_use_int() {
    let cell = Cell::new(0);
    cell.set(1);
    let i = cell.get();
    assert_eq!(i, 1)
}

pub fn cell_use_str() {
    let cell = Cell::new("aabbcc");
    cell.set("student");
    let i = cell.get();
    assert_eq!(i, "student")
}

pub fn test_refcell_multi_borrow_mut() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow_mut();
    let s2 = shard_vec.borrow_mut();
    // panicked at 'already borrowed: BorrowMutError', src\libcore\result.rs:1084:
}

pub fn test_refcell_multi_borrow_mut_1() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    shard_vec.borrow_mut().push(4);
    shard_vec.borrow_mut().push(5);
    println!("{:?}", shard_vec);
    // RefCell { value: [1, 2, 3, 4, 5] }
}

pub fn test_refcell_multi_borrow_() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow();
    let s2 = shard_vec.borrow();
    println!("{:?}", shard_vec);
    // RefCell { value: [1, 2, 3] }
}

pub fn test_refcell_borrow_and_borrow_mut() {
    let shard_vec = RefCell::new(vec![1, 2, 3]);
    let s1 = shard_vec.borrow();
    let s2 = shard_vec.borrow_mut();
    println!("{:?}", shard_vec);
    // panicked
}

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

pub fn test_rc_refcell_() {
    let rc1 = Rc::new(RefCell::new(vec![1, 2, 3]));
    rc1.borrow_mut().push(4);
    rc1.borrow_mut().push(5);

    println!("{:?}", rc1);
}

pub fn test_rc_refcell_1() {
    let rc1 = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("1strong count:{}", Rc::strong_count(&rc1));

    modify1(rc1.clone());
    println!("2strong count:{}", Rc::strong_count(&rc1));

    modify2(rc1.clone());
    println!("3strong count:{}", Rc::strong_count(&rc1));
    println!("{:?}", rc1);
    //1strong count:1
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