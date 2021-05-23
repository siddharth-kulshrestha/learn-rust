

#[derive(Debug)]
struct PointBase<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point<T,E> {
    x: T,
    y: E,
}

fn main() {
    let intPoint = PointBase { x: 9, y: 10 };
    let floatPoint = PointBase { x: 9.2, y: 10.5 };

    
    let ex1PointMix = Point { x: 9.8, y: 10 };
    let ex2PointMix = Point { x: 8, y: 10.1 };


    println!("{:?}", intPoint);
    println!("{:?}", floatPoint);
    println!("{:?}", ex1PointMix);
    println!("{:?}", ex2PointMix);
    

    println!("Hello, world!");
}
