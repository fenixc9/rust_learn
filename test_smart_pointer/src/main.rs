use std::rc::Rc;
use std::fmt::Debug;
use std::ops::Add;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod cell;
mod linked_list;

#[test]
fn t1() {
    let rc = Rc::new(32);
    assert_eq!(1, Rc::strong_count(&rc));
    let rc1 = rc.clone();
    assert_eq!(2, Rc::strong_count(&rc));
}

#[test]
fn t2() {
    let rc = Rc::new("this is a str");
    assert_eq!(rc.len(), 13);
}

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

#[test]
fn t4() {
    let numbers: Vec<_> = (0..13u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let mut local_numbers = &child_numbers[..];
        });
    }
}

#[test]
fn t5() {
    let numbers: Vec<_> = (0..100u32).collect();
    for _ in 0..10 {
        println!("{:?}", &numbers[..]);
    }
}

#[test]
fn t7() {
    cell::cell_use_int()
}

#[test]
fn t8() {
    cell::test_refcell_try_borrow()
}

#[test]
fn t9() {
    cell::test_rc_refcell_1()
}

#[test]
fn t10() {
    cell::test_refcell_panic()
}