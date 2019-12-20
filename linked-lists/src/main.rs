use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None
        }))
    }
}

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, value: String) {
        let newNode = Node::new(value);
        match self.tail.take() {
            Some(oldTail) => oldTail.borrow_mut().next = Some(newNode.clone()),
            None => self.head = Some(newNode.clone())
        };
        self.length += 1;
        self.tail = Some(newNode)
    }
}

pub fn main() {

}