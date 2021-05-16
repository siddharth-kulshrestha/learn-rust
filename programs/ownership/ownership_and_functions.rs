// Ownership rules to variables that are passed to functions:
//
// Passing a variable to a function will move or copy, just as assignment does. 
//
// When you pass a variable to a function, ownership of that variable changes to that of that
// function. 
//
// For eg:
//
// fn main() {
//    let s = String::from("hello!");
//
//    takes_ownership(s); // s value moves to takes_ownership function
//    // now s is no longer valid and takes_ownership is the owner for value of s.
//     
//     //  println!("{}" s); // it will not work
//
//
//    let x = 5; 
//    makes_copy(x); // Since integers are stack allocated here, copy of x will be passed to x.
// }
// 
// fn takes_ownership(s1: String) {
//   println!("{}", s1);
// }
//
// Return Values and Scope
//
// Returning values can also transfer ownership.
//  
//
//  Lets modify above takes_ownership() function to pass ownership back.
//
// fn main() {
//    let mut s = String::from("hello!"); // s has to be mutable.
//
//    s = takes_ownership(s); // s value moves to takes_ownership function
//    // without mut s, we can also create a new variable like,
//    // let b = takes_ownership(s); // Now b will again be owner of value returned.
//    // now s is no longer valid and takes_ownership is the owner for value of s.
//
//    let x = 5; 
//    makes_copy(x); // Since integers are stack allocated here, copy of x will be passed to x.
// }
// 
//  fn takes_ownership(s1: String) -> String {
//       
//   // do tasks with s1
//   println!("{}", s1);
//
//   // Returning s1
//   s1
//  }
//
