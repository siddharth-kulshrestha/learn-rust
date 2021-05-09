fn main() {
    
    // Rust supports three types of loops.
    //
    // 1. Loop: using loop keyword. 
    //    -> Breaking condition must be explicitly given inside the statements for the loop.
    //    Usually used to create infinite loops for threads exec etc.
    let mut n=0;
    loop {
        if n < 5 {
            println!("Hello!");
            n += 1;    
            continue;
        }
        break;
    }


    // 2. While loop: Similar to other languages
    let mut a = 0;
    while a < 10 {
        println!("Hello from while");
        a += 1;
    }

    // 3. For loop: Different from other languages, in rust for loop is used to iterate over
    //    collections.
    //    Syntax: 
    //
    //    for var_name in range {
    //                    /*Statements*/
    //    }
    for p in 1..10 {
            println!("{}", p);
    }

    let arr = [5, 10, 12, 131];
    for elem in arr.iter() {
        print!("{} ", elem);
    }
    //println!("{}", 100..105); // compile time error
}
