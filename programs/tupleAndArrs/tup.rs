

// Tuple - Compound datatype 
// can store different types of values
// Have fixed length, once declared they cannot grow or shrink in size.
// When you assign tuple to variable(s) its known as destructing. (Similar to destructing in JS)
// tuple index starts from 0.
fn main() {
    let tup = (123, 23.3, 22, "Str");
    let tup2:(i32, f64, u8) = (326, 3.2, 11);

    println!("{:?}", tup);
    println!("{:?}", tup2);

    println!("{:?}", tup.3);

    do_something(tup2);

    destructing_ex(tup2)
}


fn do_something(x: (i32, f64, u8)) {
    println!("Doing something.....");
    println!("{}", x.0);
}


fn destructing_ex(x: (i32, f64, u8)) {
    println!("Destructing example.....");

    let (q, w, e) = x;
    println!("_{}_{}_{}_", q, w, e);
}
