
fn main() {
    let a = 20;  // Stack allocated values.
    let b = a;

    println!("{} {}", a, b);

    let x = String::from("ABCD!");
    let w = x; // Here is the problem we have two owners at a time for a single memory location // Here x and w both data pointers are pointing to same memory location but rust doesn't allows it and now w will own the memory location of "ABCD!" and x will no longer be valid. It is a memory safety feature by rust to handle double free error which can lead to memory corruption or security vulnerabilities. 



    println!("w: {}",w);
 

    //println!("x: {}", x); // Error - move occurs because `x` has type `String`, which does not implement the `Copy` trait
//  We get this (move) error when we are going to use invalidated variable. Here x is invalid. And
//  We get error because Rust prevents us from using the invalidated references.

    // If we want to clone the heap allocated data (specially strings) then we can use
    // string.clone();
// For eg: 
 let s1 = String::from("Hello");
 let s2 = s1.clone();
 println!("{} , {}", s1, s2); // It will work perfectly fine!

}
