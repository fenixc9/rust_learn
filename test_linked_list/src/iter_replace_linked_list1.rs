pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let x = Box::new(
            Node {
                elem,
                next: self.head.take(),
            }
        );

        self.head = Some(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut bn) = current_link {
            current_link = self.head.take()
        }
    }
}

