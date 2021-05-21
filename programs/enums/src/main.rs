// Enums 
// allows us to define a type by enumerating its possible values.
//
// enum IpAddrKind {
//  V4,
//  V6
// }
//
// IpAddrKind is now a custom data type 
//
//
// Enum and its variants should have CamelCase.
//
// How to create instances of enums? 
//
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;
//
//
// Now we can define a function that takes any IpAddrKind instance.
// eg: 
// fn route(ip_type:IpAddrKind) { }


#[derive(Debug)] // for enable printing enums

// creating enum
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);   // We have to use `{:?}` for printing / formatting enum value.
route(four)

}

fn route(ip: IpAddrKind) {
  println!("{:?}", ip);
}
