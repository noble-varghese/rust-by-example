// -------------------------------------------
// Doubly Linked List
// -------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

struct List<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

struct Node<T> {
    element: T,
    next: Pointer<T>,
    previous: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: std::fmt::Display> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            previous: None,
        }))
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().previous = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("The list is empty");
        } else {
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().previous.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail.take();
                        println!("List is empty after removal");
                        None
                    }
                });
        }
    }

    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("The list is empty");
        } else {
            self.tail
                .take()
                .map(|old_tail| match old_tail.borrow_mut().previous.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.tail.take();
                        println!("List is empty after removal");
                        None
                    }
                });
        }
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("[]");
            return;
        } else {
            let mut traversal = self.head.clone();
            while !traversal.is_none() {
                print!("{} ", traversal.as_ref().unwrap().borrow().element);
                traversal = traversal.unwrap().borrow().next.clone();
            }
            println!("End");
        }
    }
}

fn main() {
    let mut list = List::new();
    list.remove_front();
    list.push_front(32);
    list.push_front(32);
    list.push_front(32);
    list.push_front(32);
    list.push_back(48);
    list.print();
    list.remove_front();
    list.print();
    list.remove_front();
    list.print();

    list.remove_front();
    list.print();

    list.remove_front();
    list.print();

    list.remove_front();
    list.print();

    list.remove_back();
    list.print()

}
