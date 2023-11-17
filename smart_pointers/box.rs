//! This crate is for smart pointers and how they differ from regular references 
//! They are different from regular references in the sense that they own the data they point to.

use crate::List::{Cons, Nil};
use crate::NewList::Cons;
use std::rc::Rc;



// We begin with the Box<T> smart pointer

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(4, Box::new(Cons(5, Box::new(Nil))));
    println!("The list is {:?}", list);
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));

}
// We use it when we have a data structure, say like a recursive one, that we cannot tell its size at compile 
// time. The Box<T> has a fixed size and it allocates data on the heap while it stores only the pointer data
// on the stack.
// we also use it when we have a large amount of data and want to transfer ownership making sure it is not 
// copied when we we do so. 

// Example let's take a look at the Cons list data structure popular in Lisp languages
// it gets its name from the cons function which means constructor that creates a pair. 
// a cons list means the first item is a value that points to the rest of the list because it can only create
// a pair. 

// enum List {
//     Cons(i32, List),
//     Nil
// }

// Because the List is recursive the compiler cannot tell how much size we would need for it so it won't 
// compile

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

// this will compile.

