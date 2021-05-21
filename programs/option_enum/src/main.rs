// Option enum 
// option is an enum defined by the standard library.
// 
// 
//
// Option value is used in many places because it encodes the very common scenario in which a value
// could be something or it could be nothing.
//
// This functionality prevents bugs that are extremely common in other programming languages.
//
// Rust doesn't have Null feature that many other languages have.
//
// Concept of null: a null is a value that is currently invalid or absent for some reason.
//
// Problem with null: if you try to use a null value as a not null value you'll get an error for
// some kind.
//
// 
// In rust, we have option enum. 
// Rust doesn't have nulls but it does have an enum that can encode the concept of a value being
// present or absent.
//
// This enum is defined by std lib as following: Option<T>:
//  
//  enum Option {
//      Some(T),
//      None,
//  }
//
// Option<T> enum is so useful that its even included in the prelude. (like println!())
// so are its variants like, Some and None. In program we dont have to use Option::prefix. 
// 
// Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type
// Option<T>.
//
//


fn main() {
   let num = Some(5);
   let text = Some("text");

   println!("{:?} {:?}", num, text);

   //let num2 = None; // You cant assign None without assigning type.
   let num2:Option<i32> = None; 
   let text:Option<&str> = None;

   println!("{:?} {:?}", num2, text);
}
