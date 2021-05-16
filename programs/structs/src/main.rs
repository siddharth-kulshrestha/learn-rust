
// This 
// #[derive(Debug)]
// is used to print structure other wise putting simply struct literal will result in an error.

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: i32,
}

impl User {
    fn get_user_name(&self) -> String {
            self.username.clone()
    }
}

fn build_user(email: String, username :String, age: i32) -> User {
    User{
        email,
        username,
        age,
    }
}

fn main() {

    let user1 = User{
        email: String::from("kulshresthasiddharth96@gmail.com"),
        username: String::from("Siddharth Kulshrestha"),
        age: 25,
    };

    println!("{:?}", user1);
    println!("Hello, world!");


    println!("name: {}", user1.username);
    println!("email: {}", user1.email);
    println!("age: {}", user1.age);


    //user1.age = 30; // Will throw error as age is not mutable
    
    let mut user2 = User{
        email: String::from("kulshresthasiddharth96@gmail.com"),
        username: String::from("Siddharth Kulshrestha"),
        age: 25,
    };

    user2.age = 30;

    println!("name: {}", user2.username);
    println!("email: {}", user2.email);
    println!("age: {}", user2.age);

    let user3 = build_user(String::from("asda@gmail.com"), String::from("adfad"), 40);

    println!("name: {}", user3.username);
    println!("email: {}", user3.email);
    println!("age: {}", user3.age);

    // Struct update syntax
    //
    // Used to copy values from some fields from one struct to another

    // Normal approach
    let u4 = build_user(String::from("asd1a@gmail.com"), String::from("adfad2"), 40);
    //let u5 = build_user(u4.email, String::from("adfad1232"), 40); // If we dont use .clone() then since string is stored on heap, it will treat as moved values.
    let u5 = build_user(u4.email.clone(), String::from("adfad1232"), 40); // Rectifying above error by cloning string value.
    println!("u4: {:?},\n u5: {:?}", u4, u5);

    // Now lets see how struct update syntax can help us doing the same

    let u5 = User {
        username: String::from("User5"),
        email: u4.email.clone(),
        ..u4
    };
    // we are getting all remaining fields from u4 i.e., age but NOTE that using this syntax will
    // have same result on email or any other heap stored field. for eg: 
    //
    //let u5 = User {
        //username: String::from("User5"),
        //..u4
    //}; // will result in error as u4.email will be copied from u4 and it is a string value which
    //is stored in heap so move error will occur. Struct update syntax wont clone the values.

    println!("u4: {:?},\n u5: {:?}", u4, u5);

    // Methods
    // are linked with struct
    // self will be first parameter to methods impl block can be used to define methods.
    println!("username from method: {}", u5.get_user_name())
}
