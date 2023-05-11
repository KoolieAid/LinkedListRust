#![allow(unused)]

use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

use std::fmt::Debug;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Node<T> {
    data: Rc<RefCell<T>>,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Node<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new(data: T) -> LinkedList<T> {
        let head = Node {
            data: Rc::new(RefCell::new(data)),
            next: None,
        };

        LinkedList {
            head: Some(head),
            size: 1,
        }
    }

    pub fn new_empty() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn add(&mut self, data: T) -> Result<(), &'static str>
    where
        T: Debug,
    {
        let new = Node {
            data: Rc::new(RefCell::new(data)),
            next: None,
        };

        match &self.head {
            Some(_) => {}
            None => {
                self.head = Some(new);
                self.size += 1;
                return Ok(());
            }
        }

        let mut iter: &mut Node<T> = self.head.as_mut().unwrap();

        while iter.next.is_some() {
            // println!("yo");
            iter = iter.next.as_mut().unwrap();
        }

        iter.next = Some(Box::new(new));
        self.size += 1;

        Ok(())
    }

    pub fn remove(&mut self, index: i32) -> Result<T, &'static str>
    where
        T: Debug,
    {
        let size = self.size as i32;

        match index {
            i if i < 0 => return Err("Less than 0"),
            i if i >= self.size as i32 => return Err("More than size"),
            _ => {}
        }

        if self.size == 1 {
            let data = Rc::try_unwrap(self.head.take().unwrap().data).unwrap();

            self.size -= 1;
            return Ok(data.into_inner());
        }

        let mut iter: &mut Node<T> = self.head.as_mut().ok_or("List is already empty")?;

        for i in 0..index - 1 {
            iter = iter.next.as_mut().unwrap();
        }

        let item: Box<Node<T>> = iter.next.take().ok_or("Edge case scenario idk")?;

        let data = Rc::try_unwrap(item.data).expect("item.data has multiple references");

        self.size -= 1;
        Ok(data.into_inner())
    }

    pub fn get(&mut self, index: i32) -> Result<RefMut<T>, &'static str>
    where
        T: Debug,
    {
        let size = self.size as i32;

        match index {
            i if i < 0 => return Err("Less than 0"),
            i if i >= self.size as i32 => return Err("More than size"),
            _ => {}
        }

        let mut iter: &mut Node<T> = self.head.as_mut().ok_or("Empty List")?;

        for i in 0..index {
            iter = iter.next.as_mut().unwrap();
        }

        Ok(iter.data.borrow_mut())
    }
}
