#[allow(non_camel_case_types)]
// There are three ways to specify enum values
//
// 1. Using enum as a type for fields of the structure.
// 2. Predefined values. 
 //3. Using different types inside enum.
//
//
//
//
//

// 1. Using enum as a type for fields of the structure
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V5,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 2. Predefined values
#[derive(Debug)]
enum fruits {
    apple=0, // We can only store integer types
    mango=10,
    watermelon=20,
}

 //3. Using different types inside enum.
#[derive(Debug)]
enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");

    // 1
    let a = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };


    println!("{:?}", a);

    // 2
    //
    let f = fruits::mango;
    println!("{:?}", f as i32);

    // 3
    let h = IpAddress::V4(127,0,0,1);
    let l = IpAddress::V6(String::from("localhost"));

    println!("{:?}", h);
    println!("{:?}", l);
}
