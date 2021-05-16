// Data race happens when following three behaviour occurs:
// 1. Two or more pointers access the same data at the same time. 
// 2. At least one of the pointers is being used to write to the data. (or mutating the data)
// 3. There is no mechanism being used to synchronize access to the data.

// Data races cause undefined behaviour and can be difficult to diagnose and fix when you're trying
// to track them down at runtime; 
// Rust prevents this problem from happening because it wont even compile code with data races.
//
// Dangling pointer:
//
// A pointer / reference pointing to invalid / garbage value / already deallocated value memory location.
//
// RUST is a MEMORY SAFE and it raises compile time error in case of dangling pointers.
//


fn main() {
    let a = dangle(); // error[E0106]: missing lifetime specifier

}

fn dangle() -> &String {
    let d = String::from("Hello World!");
    &d
} // Here life of `String::from("Hello World")` has ended and d holds dangling pointer which is being returned.

