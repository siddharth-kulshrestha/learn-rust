
// Rust doesn't have garbage collector but it follows another memory safety technique called as
// ownership. 

// Stack: Store values in the order it gets them and removes the values in the opposite order. LIFO
// Stack must take up a known, fixed size.
// It cannot store types whose size is unknown at compile time for example strings.



// Heap: can store data with a size unknown at compile time or size that might change instead.
//
// Heap is less organized. In heap data can be stored at anywhere. 
//
// OS finds an empty spot somewhere in the heap that is big enough and marks it as being in use.
//
// Heaps uses and returns pointer, which is the address of that location.
// Whenever we store data on heap, it returns pointer to that heap memory location.
// This process is called "allocating" on the heap.
//
// But in stack it is not called allocating as we already know the size of data that is present on
// stack.
//
// For Eg: 
// push_str()

// Why is stack is fast ?
// Stack is fast because of the way it accesses the data but in heap you have to follow a pointer
// to get there.
// Following a pointer taskes more time rather than accessing data in which is present on top of
// stack. 
//
// Stack is fast because all data on the stack must take up a known, fixed size. 
//
// Allocating large amount of space on the heap can also take time. Because we dont know size of
// memory block that needs to be allocated.
// 
//
//
// Ownership in Rust:
// Rules:
// 1. Each value in rust has a variable thats called its owner.
//    eg: let a = 10; // here a is owner of data of 10. 
// 2. There can be one onwer at a time.
// --> Basically, you cannot have two variables pointing to same memory location.
// 3. When owner goes out of scope, value will be dropped.
// eg: 
// fn main()  {
// let a = 10;
// } // Rust will deallocate memory occupied by a, here at this line.
//
// fn fun() {
//     a = 20; // unresolved name error
// }
//
// Memory and Allocation:
// 1. String types:
// In order to support mutable, growable piece of text we need to allocate an amount of memory on
// the heap, unknown at compile time, to hold the contents.
// -> The memory must be requested from OS at runtime. 
// -> We need a way of returning this memory to the OS when we are done with our string.
// 
// Since in other languages above deallocation of memory is handled by GC, the GC keeps track and
// cleans up memory that isn't being used anymore.
//
// If GC is not supported then its our responsibility to identify when memory is no longer being
// used and call code to explicitly return it.
// --> In this scenario if programmer has returned memory too early then the program will have some
// bugs.
// And if you are going to deallocate memory later on then you will be just wasting out resources.
// So memory should be deallocated at proper time.
//
// Rust takes different path: the memory is automatically returned once the variable that owns it
// goes out of scope.
// for eg: 
// fn main() {
//    let s = String::from("Hello");
//
//    // do stuff with s
// } // this scope is now over and s is no longer valid or needed.
//
// When a variable goes out of scope rust calls a special function for us i.e., drop().
// And RUST calls drop() automatically at the CLOSING BRACKET ("}").
// 
// Understanding "move"
//
// eg:
//
// fn main() {
//  let a = 20;
//  let b = a;
//
//  println!("{} {}", a, b); // Everything will work fine and value of a will be printed.
// }
//
// fn main() {
//  let s1 = String::from("hello");
//  let s2 = s1;
//
//  println!("{} {}", s1, s2); // Error use of moved value.
//  // if we remove s1 from print statement then program will work fine.
//  // println("{}", s2); // will work perfectly fine. 
// }
//
//
