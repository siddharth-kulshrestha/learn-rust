// If let
//
// With match we need to write a lot of code that is useless because if we want to match  a pattern
// then we have to write that underscore (_) condition and code that we want to execute. 
// Everytime doing this is not efficient and not effective.
//
// So we can use if let statement to do the same in idiomatic way.
//
//

fn check_for_some(val: Option<u8>) {
    if let Some(3) = val {
        println!("three!")
    } else if let Some(4) = val {
        println!("four!!")
    } else {
        println!("default value!!");
    }
}


fn main() {
    let some_u8 = Some(3);
    check_for_some(some_u8);

    let some_u8_2 = Some(4);
    check_for_some(some_u8_2);

    let some_u8_3 = Some(8);
    check_for_some(some_u8_3);
}
