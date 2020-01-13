use std::mem;

pub struct List {
    head: Link
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let x = Box::new(
            Node {
                elem,
                next: mem::replace(
                    &mut self.head,
                    None,
                ),
            }
        );

        self.head = Some(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            Some(b) => {
                self.head = b.next;
                Some(b.elem)
            }
            None => {
                None
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, None);
        while let Some(mut bn) = current_link {
            current_link = mem::replace(&mut self.head, None)
        }
    }
}

