use std::io; // importing another module (io) from standard library (std)
fn main() {
let mut a = String::new(); // Created a normal string
println!("Enter a string!");
io::stdin().read_line(&mut a).expect("Failed");

let a:i32 = a.trim().parse().expect("Failed to parse!");
println!("{}",a)
}


// module is collection of methods
