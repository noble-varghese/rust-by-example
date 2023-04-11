/*
   Ref cel smart pointer:
   Checking borrowing rules at compile time is default to rust. Borrowing rules with RefCell is done at runtime and not at compile time.
   - rust rules doesn't allow to have mutable references to a variable that is immutable to itself. RefCell pointer allows us to do it.
*/

use std::cell::{Ref, RefCell};
use std::rc::Rc;

fn main() {
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}", x1, x2);

    let a = RefCell::new(50);
    let b = a.borrow();
    let c = a.borrow();

    drop(b);
    drop(c);
    let d = a.borrow_mut();

    drop(d);
    println!("{:?}", a);

    let f = RefCell::new(43);
    let mut g = f.borrow_mut();
    *g = 78;
    drop(g);
    *f.borrow_mut() = 16;
    println!("{:?}", f);

    // Combination of RefCell and Rc
    let h = Rc::new(RefCell::new(String::from("Robin")));
    let j = Rc::clone(&h);
    *j.borrow_mut() = String::from("Robin");
    println!("{:?}", h);
}
