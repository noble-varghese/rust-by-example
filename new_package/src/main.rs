// use new_package;
// // use new_package::file_1;
// // use new_package::file_2;
// use array_tool::vec::*;

// fn main() {
//     new_package::printing();
//     println!("Hello, world!");
//     let len = 43;
//     let breadth = 56;
//     new_package::file_1::rect_area(&len, &breadth);
//     new_package::file_2::printing();

//     let vec_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 8];
//     let vec_2 = vec![1, 2, 3, 4, 5];
//     let intersection = vec_1.intersect(vec_2.clone());
//     println!(
//         "The intersection of the two vectors are: {:?}",
//         intersection
//     );

//     let unique = vec_1.union(vec_2);
//     println!("The unique vector from vec_1 are: {:?}", unique);
// }

/*
   Smart Pointers
       - pointers with metadata and other special capabilities.
       - Reference counting smart pointers that track the number of owners and remove them from memory whenever it goes out of scope.
       - Box Smart pointer.

    When are box pointers useful ?
    - They're used to store the values in the heap.
*/

// fn main() {
//     let single_value = Box::new(0.625); // The variable is stored in stack and value is stored in heap.
//     let x = 0.625; // This is stored in the stack
//     println!("Are the value being equal {}", x == *single_value);
//     let mut stack_var = 4;
//     let _stack_ref = &stack_var;
//     let heap_var = Box::new(stack_var); // A copy of stack_var will be stored in the heap and the heap_var will point to the memory in heap.
//     stack_var = 7;
//     println!(
//         "The value of stack_var = {} and heap_var = {}",
//         stack_var, heap_var
//     );
// }

/*
   Box pointers and when to use it.
   - Box pointers allow you to store the data in heap rather than on the stack.
   - When you want to transfer ownership without copying the data.
   - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
   - Used to store values who's size cannot be determined in the compile time.
*/

// #[derive(Debug)]
// enum List {
//     Cons(i32, Option<Box<List>>),
// }

// use List::Cons;

// fn main() {
//     let list = Cons(4, Some(Box::new(Cons(3, Some(Box::new(Cons(2, None)))))));
//     println!("{:?}", list);
// }

/*

   Custom Smart pointer creation:
   -
*/

// use std::ops::Deref;

// struct MySmartPointer {
//     value: i32,
// }

// impl Deref for MySmartPointer {
//     type Target = i32;
//     fn deref(&self) -> &i32 {
//         &self.value
//     }
// }
// impl Drop for MySmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping mysmartpointer from the memory {:?}", self.value);
//     }
// }

// impl MySmartPointer {
//     fn new(x: i32) -> MySmartPointer {
//         MySmartPointer { value: x }
//     }
// }

// fn main() {
//     let a = 50;
//     let b = Box::new(a);
//     println!("{}", 50 == a);
//     println!("{}", 50 == *b); // Deref operator. Box implements the deref trait in order to access the value on the memory location.
//                               // println!("{}", a == b);

//     let sptr1 = MySmartPointer::new(a);
//     let sptr2 = MySmartPointer::new(a+3);
//     let sptr3 = MySmartPointer::new(a+5);
//     // let sptr_2 = MySmartPointer::new(10);
//     println!("{}", a == *sptr1);

//     drop(sptr1);
// }

use std::collections::linked_list;

/*
   LinkedList in rust
*/
#[derive(Debug)]
struct LinkedList {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

impl LinkedList {
    fn create_empty_linkedlist() -> LinkedList {
        LinkedList { head: None }
    }

    fn add(&mut self, element: i32) {
        //Note: a match will transfer the ownership. So we cannot do that here.
        // match self.head {
        //     None => {
        //         self.head = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }))
        //     }
        //     Some(previous_head) => {
        //         let new_head = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //         self.head = new_head
        //     }
        // }

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
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(head) => Some(head.element),
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
                },
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
