/*
    RC Reference Pointer
*/
use std::rc::Rc;

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn make_rc() -> Rc<String> {
    let s1 = Rc::new(String::from("Noble"));
    println!(
        "Count when the pointer is created {}",
        Rc::strong_count(&s1)
    );
    let s2 = s1.clone();
    println!("Counter after the clone is created {}", Rc::strong_count(&s2));
    s2
}

fn main() {
    let a = Rc::new(Cons(
        100,
        Rc::new(Cons(200, Rc::new(Cons(300, Rc::new(Nil))))),
    ));
    println!("After a: {:?}", Rc::strong_count(&a));
    {
        // Here the counter will be incremented from the previous values and the pointer will be removed once the code block ends and thus resetting the counter of the rc pointer.
        let _b = Rc::new(Cons(3, Rc::clone(&a)));
        println!("After b: {:?}", Rc::strong_count(&a));

        let _c = Rc::new(Cons(500, Rc::clone(&a)));
        println!("After c: {:?}", Rc::strong_count(&a));
    }

    //After the code block the reference counter becomes 1 as the other counter pointers will be removed after the code block.
    println!("After code block: {:?}", Rc::strong_count(&a));

    println!("New rc pointer function");
    let s2 = make_rc();
    println!("The new rc pointer is {:?}", s2);
}
