use std::io;
fn main() {

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("Enter a: ");
    io::stdin().read_line(&mut a).expect("Failed to read stdin");
    println!("Enter b:");
    io::stdin().read_line(&mut b).expect("Failed to read stdin");
    println!("Enter c: ");
    io::stdin().read_line(&mut c).expect("Failed to read stdin");


    let a:i64 = a.trim().parse().expect("Failed to parse!");
    let b:i64 = b.trim().parse().expect("Failed to parse!");
    let c:i64 = c.trim().parse().expect("Failed to parse!");

    if a > b && a > c {
        println!("a is greatest!");
    } else if b > a && b > c {
        println!("b is greatest");
    } else {
        println!("c is greatest");
    }


}
