use std::io;
fn main() {
let mut a = String::new();
println!("Enter a number!");
io::stdin().read_line(&mut a).expect("Failed");
let a:i32 = a.trim().parse().expect("Failed to parse");
println!("{}",a)
}
