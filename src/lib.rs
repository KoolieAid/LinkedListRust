use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Node<T> {
    data: Rc<T>,
    child: Option<RefCell<Box<Node<T>>>>,
}

impl<T> Node<T> {
    fn generate_single(data: T) -> Node<T> {
        Node {
            data: Rc::new(data),
            child: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Node<T>,
    count: i32,
}

impl<T> LinkedList<T> {
    pub fn new(first: T) -> LinkedList<T> {
        let node = Node::generate_single(first);

        LinkedList {
            head: node,
            count: 1,
        }
    }

    pub fn add(&mut self, element: T) {
        todo!("Please help me");
        let new_node = Node::generate_single(element);

        if let None = self.head.child {
            return;
        }

        let mut current: Node<T>;

//        while *(self.head.child.borrow_mut().) != None {
//            println!("please help");
//        }
    }

}

impl<T> std::ops::Index<i32> for LinkedList<T> {
    type Output = T;
    fn index(&self, index: i32) -> &Self::Output {
        &self.head.data
    }
}
