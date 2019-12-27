use std::cell::RefCell;
use std::rc::Rc;

// gives the Clone functionality
#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: Link,
    prev: Link // uses Rc<RefCell>> for ownership
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            prev: None,
            next: None
        }))
    }
}

#[derive(Clone, Debug)]
struct TransactionLog {
    head: Link,
    tail: Link,
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

pub struct ListIterator {
    current: Link
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()  
            },
            None => None
        };
        result
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