#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug + std::marker::Copy> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl <T: std::fmt::Debug + std::marker::Copy> LinkedList<T>{
    fn create_empty_linkedlist() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.element),
            None => None,
        }
    }

    fn printing(&self) {
        let mut list_traversal = &self.head;
        while true {
            match list_traversal {
                Some(node) => {
                    println!("{:?}", node.element);
                    list_traversal = &node.next;
                }
                None => break,
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::create_empty_linkedlist();
    list.add(7);
    list.add(78);
    list.add(45);
    list.add(99);
    list.printing();
    println!("The linkedlist is \n{:?}", list);
    println!("The peek element is: {:?}", list.peek());

    list.printing();
}
