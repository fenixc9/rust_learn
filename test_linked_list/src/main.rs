use std::borrow::Borrow;
use std::cell::Ref;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};

mod mem_replace_linked_list;
mod mem_replace_linked_list1;
mod take_replace_linked_list1;
mod rc_linked_list;
mod rc_refcell_deque;
mod unsafe_linked_list;

#[test]
fn t1() {
    let mut list = mem_replace_linked_list::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.pop().unwrap(), 2);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

#[test]
fn t2() {
    let mut list = mem_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.pop().unwrap(), 2);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

#[test]
fn t3() {
    let mut list = take_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.pop().unwrap(), 2);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}


#[test]
fn t4_peek() {
    let mut list = take_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.peek(), Some(&2));
}

#[test]
fn t5_peek_mut() {
    let mut list = take_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert_eq!(list.pop().unwrap(), 4);
    assert_eq!(list.pop().unwrap(), 3);
    assert_eq!(list.peek_mut(), Some(&mut 2));
    let option = list.peek_mut().unwrap();
    *option = 42;
    assert_eq!(list.peek(), Some(&42));
}

#[test]
fn t6_into_iter() {
    let mut list = take_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    for x in list.into_iter() {
        println!("{}", x);
    }
}

#[test]
fn t7_into_iter() {
    let mut list = take_replace_linked_list1::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    let iter = list.iter();
    for x in iter {
        println!("{}", x);
    }
}

#[test]
fn t8_rc_linked_list() {
    let mut list = rc_linked_list::List::new();
    let ret = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    assert_eq!(ret.head(), Some(&4));
}


#[test]
fn t9_rc_linked_list() {
    let mut list = rc_linked_list::List::new();
    let ret = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    assert_eq!(ret.head(), Some(&4));
}


#[test]
fn t10_rc_linked_list() {
    let mut list = rc_linked_list::List::new();
    let ret = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    let ret = ret.tail();
    assert_eq!(ret.head(), Some(&3));

    let ret = ret.tail();
    assert_eq!(ret.head(), Some(&2));
}

#[test]
fn t11_rc_linked_list_iter() {
    let mut list = rc_linked_list::List::new();
    let ret = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    for x in ret.iter() {
        println!("{}", x);
    }
}

#[test]
fn t12_rc_linked_list_drop() {
    let mut list = rc_linked_list::List::new();
    let ret = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    println!("{:?}", list);
}

fn modify_array(mut arr: [i32; 5]) {
    arr[0] = 100;

    println!("modified array {:?}", arr);
}

#[test]
fn t13_array() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    modify_array(xs);

    println!("origin array {:?}", xs);
}

fn mut_array(a: &mut [i32]) {
    a[1] = 5;
    println!("len {}", a.len());
}

#[test]
fn t14() {
    let mut v: [i32; 3] = [1, 2, 3];
    {
        let s = &mut v;
        mut_array(s);
    }
    println!("{:?}", v);

    println!("{:?}", size_of::<&[i32; 3]>());

    println!("{:?}", size_of::<&[i32]>());
}

#[test]
fn t15() {
    let mut list = rc_refcell_deque::List::new(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);
    list.push_front(6);

    assert_eq!(list.pop_front(), Some(6));
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
}


#[test]
fn t16() {
    let mut list = rc_refcell_deque::List::new(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    assert_eq!(*list.peek_front().unwrap(), 4);
    assert_eq!(*list.peek_front().unwrap(), 4);
    assert_eq!(*list.peek_front().unwrap(), 4);
}

#[test]
fn t16_peek_front_mut() {
    let mut list = rc_refcell_deque::List::new(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    let ref_i32 = list.peek_mut_front().unwrap();
    let x = ref_i32.deref();
    assert_eq!(x, &4);
}

#[test]
fn t17_peek_front_mut() {
    let mut list = rc_refcell_deque::List::new(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    let ref_i32 = list.peek_mut_front().unwrap();
    let x = ref_i32.deref();
    assert_eq!(x, &4);
}

#[test]
fn t18_peek_back_mut() {
    let mut list = rc_refcell_deque::List::new(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

//    let ref_i32 = list.peek_back_mut().unwrap();
//    let mut x = *ref_i32.deref();
//    assert_eq!(x, 1);
//    x = 20;
//    let ref_i32 = list.peek_back_mut().unwrap();
//    let mut x = *ref_i32.deref();
//    assert_eq!(x, 20);
}

#[test]
fn t19_unsafe() {
    let mut list = unsafe_linked_list::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));
}