//! This crate is for smart pointers and how they differ from regular references 
//! They are different from regular references in the sense that they own the data they point to.


// We begin with the Box<T> smart pointer

// We use it when we have a data structure, say like a recursive one, that we cannot tell its size at compile 
// time. The Box<T> has a fixed size and it allocates data on the heap while it stores only the pointer data
// on the stack.
