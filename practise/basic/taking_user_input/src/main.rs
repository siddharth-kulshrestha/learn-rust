use std::io;

fn main() {
    
    let mut ip = String::new();
    io::stdin()
        .read_line(&mut ip)
        .expect("read line failed.");

    println!("Your input: {ip}");

}
