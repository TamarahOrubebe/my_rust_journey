use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a =  Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

     {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}



// Next is the  REFERENCE COUNTING Rc<T> smart pointer. This is used to enable data stored on the heap to have multiple owners
// It keeps track of each reference to the data and only alloows the data to be dropped when the reference
// count is 0

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

// this code will not compile because the Box takes ownership of the data we pass into it so list a would not 
// be availableto be used in the list c because list b has taken ownership of it. 

// to make that work we need the Rc<T> 

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}