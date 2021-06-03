
pub trait Animal {
    fn speaks(&self) -> String;
}

pub struct Dog {
    pub speak: String,
    pub name: String,
}

impl Animal for Cat {
    fn speaks(&self) -> String {
        format!("Name: {}\nspeak: {}\nsays: Meoww!", self.name, self.speak)
    }
}

impl Animal for Dog {  
    fn speaks(&self) -> String {
        format!("Name: {}\nspeak: {}\nsays: Bow Bow!", self.name, self.speak)
    }
}

pub struct Cat {
    pub speak: String,
    pub name: String,
}


fn main() {
    println!("Example of trait is executing:");
    
    let dg = Dog{name: String::from("Tommy"), speak: String::from("Spanish")};
    let cat = Cat{name: String::from("Billy"), speak: String::from("Japanese")};

    println!("{}\n\n", dg.speaks());
    println!("{}\n\n", cat.speaks());


}
