// Array values should be of same type
// Length is fixed
// starting index position is 0
fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let b:[i32;5] = [5, 6, 7, 8, 9];
    let x:[i32;5] = [99;5]; // Filling this array with five 99's // [99,99,99,99,99]
    
   
    a[0] = 12;

    print_arrs(a);
    print_arrs(b);
    print_arrs_via_iter(x);

    println!("END")
}

fn print_arrs(q:[i32;5]) {
    println!("length of array: {}", q.len());
    for n in 0 .. 5 {
        println!("{}", q[n]);
    }
}

fn print_arrs_via_iter(q:[i32;5]) {
    println!("length of array: {}", q.len());
    for n in q.iter() {
        println!("{}", n);
    }
}

// Memory safety feature in rust doesn't allow us to create uninitialized variables. 
// For ex: let mut a:[i:32;5];
// wont be possible, it will throw error on compile time.
