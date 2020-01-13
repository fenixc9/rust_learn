use std::borrow::{Borrow, BorrowMut};
use std::collections::LinkedList;
use std::ops::Deref;

fn main() {}

//
//struct LinkedList<T> {
//    head: Option<Node<T>>,
////    tail: Option<Node<T>>,
//}
//
//impl<T> LinkedList<T> {
//    fn new() -> Self {
//        LinkedList {
//            head: None,
////            tail: None
//        }
//    }
//
//    fn push(&mut self, t: T) {
//        match &self.head {
//            Some(mut n) => {
//                n.add_next(t)
//            }
//            None => {
//                &self.head = Node::new(t)
//            }
//        }
//    }
//}

struct Node<T: Copy> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T: Copy> Node<T> {
    fn new(v: T) -> Self {
        Node { next: None, value: v }
    }

    fn add_next(&mut self, n: Node<T>) {
        self.next = Option::Some(
            Box::new(
                n
            )
        );
    }
}

impl<T: Copy> Iterator for Node<T> {
    type Item = T;

    fn next(mut self) -> Option<Self::Item> {
        match self.next {
            Some(mut t) => {
                self.value = t.value
            }
            None => {
                None
            }
        }
    }
}

#[test]
fn test1() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let mut node4 = Node::new(4);
    node3.add_next(node4);
    node2.add_next(node3);
    node1.add_next(node2);

//    assert_eq!(node1.next.unwrap().value,2);

//    for x in node1 {
//        println!("foreach:{}");
//    }
    let leak1 = Box::leak("13");
}

#[test]
fn test2() {
    let v1 = vec![1, 2, 3];

    for x in v1 {
        println!("Got: {}", x);
    }
}


#[test]
fn test3() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(6);
    for x in list {
        println!("{}", x);
    }
}