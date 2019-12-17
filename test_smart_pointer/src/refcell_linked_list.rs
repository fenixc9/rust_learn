use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct RefCellLinkedList {}

type NextNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Self {
        Node { value: t, next: None }
    }

    fn set_next_node(&mut self, node: Option<Self>) {
        self.next = None;
        if let Some(x) = node {
            self.next = Some(Rc::new(RefCell::new(x)));
        }
    }
}

pub fn test_ref_cell_list() {
    let mut node = Node::new(1);
    node.set_next_node(Some(Node::new(2)));
    node.set_next_node(Some(Node::new(3)));
    node.set_next_node(Some(Node::new(4)));

    for x in node {
        println!("value :{:?}",x);
    }
}