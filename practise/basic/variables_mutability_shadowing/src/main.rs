fn main() {
    let mut x = 1;
    let y = 2;


    println!("value of x: {x}");
    println!("value of y: {y}");

    x = 3;
    println!("value of x: {x}");

    // y = 9; cannot assign twice to immutable variable....
    // println!("value of y: {y}");
    

    // But we can shadow a immutable variable
    let y = 9;

    println!("value of y: {y}");

    let z = 10; 
    println!("value of z is: {z}");


    {
        let z = z + 10;
        println!("value of z is: {z}");
    }
        println!("value of z is: {z}");

    // We can also use the different variable type with the same name
    let spaces = "    ";
    println!("value of spaces: {spaces}");
    let spaces = spaces.len();
    println!("value of spaces: {spaces}");

}

