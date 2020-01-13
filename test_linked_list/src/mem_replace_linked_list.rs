use std::mem;

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let x = Box::new(
            Node {
                elem,
                next: mem::replace(
                    &mut self.head,
                    Link::Empty,
                ),
            }
        );

        self.head = Link::More(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(b) => {
                self.head = b.next;
                Some(b.elem)
            }
            Link::Empty => {
                None
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut bn) = current_link {
            current_link = mem::replace(&mut self.head, Link::Empty)
        }
    }
}

