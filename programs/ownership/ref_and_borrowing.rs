// Everytime keeping check on ownership rules is not easy.
// Ownership allows us to keep memory safety in programs.
//
//
// References
//
// Using references we can refer a value without taking ownership of it.
//
// '&' ampersands are references.
//
// '&' sign allow you to refer to some value without taking ownership of it.
//
// For eg:
//  
//  let s1 = String::from("Hello world!");
//  take_ownership(&s1); // here only reference will be passed to print function
//  // and function will not take its ownership.
//
//   fn take_ownership(s1 : &String) {}
// 
//  The scope in which the reference variable is valid is the same as any function parameter's
//  scope.
//
//  We dont drop what the reference points to when it goes out of scope because we don't have
//  ownership.


fn main() {

    let s = String::from("Hello world!");
    
    let b = take_ownership(s);

    //println!("{}", s); // Error: move occurs because `s` has type `String`, which does not implement the `Copy` trait


    println!("{}", b);



    let p = String::from("Hello world 2!");
    
    will_not_take_ownership(&p);

    println!("{}", p);
}


fn take_ownership(s1: String) -> String {
    println!("s1: {}", s1);
    s1
}

fn will_not_take_ownership(s1: &String) {
    println!("s1: {}", s1);
// Here s1's value will not be dropped after this function body is returned.
// As we havent taken ownership of value against s1 so we dont have to return any value.
}
