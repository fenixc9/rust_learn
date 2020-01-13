use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> List<T> {
    pub fn new(elem: T) -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head
            .take()
            .map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
            })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| {
                Ref::map(node.borrow(), |node| &node.val)
            })
    }

    pub fn peek_mut_front(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| {
                RefMut::map(node.borrow_mut(), |node| &mut node.val)
            })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| {
                RefMut::map(node.borrow_mut(), |node| &mut node.val)
            })
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node {
                    val: elem,
                    prev: None,
                    next: None,
                }
            )
        )
    }
}