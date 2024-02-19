#![allow(dead_code)]

use core::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;


struct Node<T>{
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
    value: T
}


struct List<T>{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}


impl<T: PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Node<T>) -> bool{
        self.value == other.value
    }
}


impl<T: Display> Display for Node<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl<T: Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}


impl<T: PartialEq> PartialEq for List<T>{
    fn eq(&self, other: &Self) -> bool {
        let mut n1 = self.head.clone();
        let mut n2 = other.head.clone();

        while n1.is_some() && n2.is_some(){
            if n1.as_ref().unwrap().borrow().value != n2.as_ref().unwrap().borrow().value{
                return false;
            }
            n1 = n1.unwrap().borrow().next.clone();
            n2 = n2.unwrap().borrow().next.clone();
        }

        n1.is_none() && n2.is_none()
    }
}


impl<T: Debug> Debug for List<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n1 = self.head.clone();
        let mut buffer = String::new();
        while let Some(somen1) = n1{
            buffer.push_str(format!("{:?}", somen1.borrow().value).as_str());
            n1 = somen1.borrow().next.clone();
        }
        write!(f, "{}", buffer)
    }
}


impl<T> Node<T>{
    fn new(el: T) -> Self{
        Node{prev: None, next: None, value: el}
    }
}


impl<T: Debug> List<T>{
    fn new() -> Self{
        List{head: None, tail: None, size: 0}
    }

    fn print_list(&self){
        println!("{:?}", self);
    }

    fn push(&mut self, el: T){
        let new_node = Rc::new(RefCell::new(Node::new(el)));
        match self.head.take(){
            Some(old_head) => {
                old_head.as_ref().borrow_mut().prev = Some(new_node.clone());
                new_node.as_ref().borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let head_node = self.head.take()?;
        if let Some(ref new_head) = head_node.borrow().next {
            new_head.as_ref().borrow_mut().prev = None;
            self.head = Some(new_head.clone());
        } else {
            self.head = None;
            self.tail = None;
        }
        self.size -= 1;
        Some(Rc::try_unwrap(head_node).ok()?.into_inner().value)
    }

    fn push_back(&mut self, el: T){
        let new_node = Rc::new(RefCell::new(Node::new(el)));
        match self.tail.take(){
            Some(old_tail) => {
                old_tail.as_ref().borrow_mut().next = Some(new_node.clone());
                new_node.as_ref().borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        let tail_node = self.tail.take()?;
        if let Some(ref new_tail) = tail_node.borrow().prev {
            new_tail.as_ref().borrow_mut().next = None;
            self.tail = Some(new_tail.clone());
        } else {
            self.head = None;
            self.tail = None;
        }
        self.size -= 1;
        Some(Rc::try_unwrap(tail_node).ok()?.into_inner().value)
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_eq() {
        let node1 = Node {
            prev: None,
            next: None,
            value: 5,
        };

        let node2 = Node {
            prev: None,
            next: None,
            value: 5,
        };

        assert_eq!(node1, node2);
    }

    #[test]
    fn test_node_display() {
        let node = Node {
            prev: None,
            next: None,
            value: 5,
        };

        assert_eq!(format!("{}", node), "5");
    }

    #[test]
    fn test_list_eq() {
        let node1 = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value: 5,
        }));

        let node2 = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value: 5,
        }));

        let list1 = List {
            head: Some(node1.clone()),
            tail: Some(node1),
            size: 1,
        };

        let list2 = List {
            head: Some(node2.clone()),
            tail: Some(node2),
            size: 1,
        };

        assert_eq!(list1, list2);
    }

    #[test]
    fn test_list_debug() {
        let node = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value: 5,
        }));

        let list = List {
            head: Some(node.clone()),
            tail: Some(node),
            size: 1,
        };

        assert_eq!(format!("{:?}", list), "5");
    }


    #[test]
    fn test_push() {
        let mut list = List::new(); // 15 10 5
        list.push(5);
        list.push(10);
        list.push(15);
        assert_eq!(format!("{:?}", list), "15105");
    }

    #[test]
    fn test_pop() {
        let mut list = List::new(); // 5 10 15
        list.push_back(5);
        list.push_back(10);
        list.push_back(15);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), Some(15));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push_back() {
        let mut list = List::new(); // 5 10 15
        list.push_back(5);
        list.push_back(10);
        list.push_back(15);
        assert_eq!(format!("{:?}", list), "51015");
    }

    #[test]
    fn test_pop_back() {
        let mut list = List::new(); // 5 10 15
        list.push_back(5);
        list.push_back(10);
        list.push_back(15);
        assert_eq!(list.pop_back(), Some(15));
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), None);
    }
}