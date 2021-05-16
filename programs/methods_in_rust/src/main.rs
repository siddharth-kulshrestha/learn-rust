//
// Associated functions:
// 
// When we define a function within impl block that dont take self as parameter it is called
// associated functions.
//
// They are associated with the struct. But 
// they are functions not methods as they dont require instance of the struct to work with.  for
// eg:
//  String::from()
//
//  struct_name::associated_func()
//
// Associated functions are often used for constructors that will return a new instance of the
// struct.

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn are_equal(&self, other_rect:Rectangle) -> bool {
        self.width == other_rect.width && self.height == other_rect.height
    }

    fn square(size: i32) -> Rectangle { // Associated function
        Rectangle {
            width: size,
            height: size,
        }
    }

}


fn main() {
    println!("Hello, world!");
    
    let r1 = Rectangle::square(14);
    let r2 = Rectangle {
        width: 14,
        height: 14,
    };
    
    println!("{}", r1.are_equal(r2));

}
