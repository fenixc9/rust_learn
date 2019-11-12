use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::net::Shutdown::Read;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::linked_list::my_link_list::List;

mod my_link_list {
    type NextNode<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    pub struct Node<T> {
        value: T,
        next: NextNode<T>,
    }

    impl<T> Node<T> {
        fn new(elem: T) -> Self {
            Node {
                value: elem,
                next: None,
            }
        }

        fn set_next(&mut self, node: Option<Self>) {
            self.next = None;
            if let Some(x) = node {
                self.next = Some(Box::new(x));
            }
        }

        fn get_next(&mut self) -> Option<&mut Self> {
            if let Some(ref mut x) = self.next {
                return Some(x);
            }
            None
        }

        fn get(&mut self, index: usize) -> Option<&mut Self> {
            if index == 0 {
                return Some(self);
            }
            if let Some(x) = self.get_next() {
                x.get(index - 1)
            } else {
                None
            }
        }

        fn get_last(&mut self) -> &mut Self {
            if let Some(ref mut x) = self.next {
                return x.get_last();
            }
            self
        }

        fn get_last_immutable(&self) -> &Self {
            if let Some(ref x) = self.next {
                return x.get_last_immutable();
            }
            self
        }

        fn get_value(&self) -> &T {
            &self.value
        }

        fn push(&mut self, elem: T) {
            let new_node = Node::new(elem);
            self.get_last().set_next(Some(new_node));
        }
    }

    #[derive(Clone, Debug)]
    pub struct List<T> {
        len: usize,
        next: NextNode<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { len: 0, next: None }
        }

        pub fn get_next_mut(&mut self) -> Option<&mut Node<T>> {
            if let Some(ref mut x) = self.next {
                return x.get_next_mut();
            }
            None
        }


        pub fn get_next(&self) -> Option<&Node<T>> {
            if let Some(mut x) = &self.next {
                return x.get_next();
            }
            None
        }

        pub fn get(&mut self, index: usize) -> Option<&mut Node<T>> {
            if index > self.len || index == 0 {
                return None;
            }

            let node = self.get_next_mut().unwrap();
            if index == 1 {
                return Some(node);
            }

            node.get(index - 1)
        }

        pub fn get_last(&mut self) -> Option<&mut Node<T>> {
            if let Some(ref mut x) = self.next {
                Some(x.get_last())
            } else {
                None
            }
        }

        pub fn get_last_immutable(&self) -> Option<&Node<T>> {
            if let Some(ref x) = self.next {
                Some(x.get_last_immutable())
            } else {
                None
            }
        }

        pub fn get_last_value(&self) -> Option<&T> {
            if self.len == 0 {
                return None;
            }
            Some(self.get_last_immutable().unwrap().get_value())
        }

        pub fn push(&mut self, elem: T) {
            if self.len == 0 {
                self.next = Some(Box::new(Node::new(elem)));
            } else {
                if let Some(ref mut x) = self.get_last() {
                    x.push(elem);
                }
            }
            self.len += 1;
        }

        pub fn pop(&mut self) {
            if self.len == 0 {
                return ();
            }
            self.len -= 1;
            let index = self.len;
            self.get(index - 1).unwrap().set_next(None);
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = my_link_list::Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
//        if let Some(x) = self.get_next() {
//            let x1 = x.;
//            return Some(x.deref())
//        }
//        return None
        let Some(o) = self.get_next();
    }
}

pub fn test_linked_list() {
    let mut my_list = List::<u8>::new();
    my_list.push(0);
    my_list.push(1);
    my_list.push(2);
    my_list.push(3);
    my_list.push(4);

    for x in my_list {
        println!("{:?}", x);
    }
}