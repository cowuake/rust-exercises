use std::cell::RefCell;
use std::rc::Rc;

// This serves the implementation of a simple (not double) linked list,
//     hence there's no pointer to the previous node!
struct Node<T> {
    value: T, // Value stored at the node
    next: Option<Rc<RefCell<Node<T>>>> // Address of the next node
}

impl<T> Node<T> {
    fn new(value: T) -> Self { // Constructor
	Self {
	    value,
	    next: None,
	}
    }

    fn point_to(&mut self, value: Rc<RefCell<Node<T>>>) {
	self.next = Some(value);
    }
}

pub struct Llist<T> {
    head: Option<Rc<RefCell<Node<T>>>>, // First node in the linked list
    tail: Option<Rc<RefCell<Node<T>>>>, // Last node in the linked list
    len: usize, // Length of the linked list (number of nodes)
}

impl<T> Llist<T> {
    pub fn new() -> Self { // Constructor with no arguments
	Self {
	    head: None,
	    tail: None,
	    len: 0,
	}
    }

   //pub fn push(&mut self, value: T) {
   //	let node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node::new(value)));
   //
   //	match self.tail {
   //	    Some(t) => t.point_to(node),
   //	    None => self.head = Some(node),
   //	}
   //	self.tail = Some(node);
   //	self.len += 1;
   //}
}
