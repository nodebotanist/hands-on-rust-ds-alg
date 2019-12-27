use std::cell::RefCell;
use std::rc::Rc;

// gives the Clone functionality
#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>> // uses Rc<RefCell>> for ownership
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
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone())
        };
        self.length += 1;
        self.tail = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong unwrapping the list Node")
                .into_inner()
                .value
        })
    }
}

pub fn main() {
    let mut log = TransactionLog::new_empty();
    log.append(String::from("Hello"));
    log.append(String::from("World!"));
    match log.pop() {
        Some(text) => println!("{}", text),
        None => println!("No list item!")
    }
    match log.pop() {
        Some(text) => println!("{}", text),
        None => println!("No list item!")
    }
}