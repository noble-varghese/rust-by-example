#[derive(Debug)]
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl LinkedList {
    fn create_empty_linkedlist() -> LinkedList {
        LinkedList { head: None }
    }

    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<i32> {
        let previous_head = self.head.take();
        match previous_head {
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<i32> {
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
                    println!("{}", node.element);
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
