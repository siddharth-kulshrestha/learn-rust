
struct Point<T> {
    x:T,
    y:T,

}

impl <T> Point<T> {
    fn x (&self) -> &T {
        println!("calling x() in T!!!!");
        &self.x
    }
}

impl Point<f32> {
    fn number(&self) -> f32 {
        println!("calling number() in f32!!!!");
        self.x
    }
}

impl Point<i32> {
    fn number(&self) -> i32 {
        println!("calling number() in i32!!!!");
        self.x
    }
}

fn main() {
    println!("Hello, world!");
    let point = Point{x:5, y:10};
    println!("{:?}", point.x());
    
    println!("{}", point.number());
    
    let pointf = Point{x:2.2, y:1.1};
    println!("{}", pointf.number());
    



}
