fn main() {
    let mut a = String::from("Hi, "); // string, growable, built on top of str slice
    let b = "Hello"; // str slice, not growable core language type in rust

    a.push_str("How are you?");
    println!("{}", a);
    println!("{}", b);
}
