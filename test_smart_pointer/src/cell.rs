use std::cell::{Cell, RefCell};

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