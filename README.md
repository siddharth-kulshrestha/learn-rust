# Rust Notes

## Why RUST ?
- Fast & secure
- Compiler will never allow you to compile code if it contains memory leaks, security bugs.
- Compiler will not allow you to compile code and lead to memory crashes.
- Other various features like no-gc, no null pointer reference and cool set of ownership features.
- Resources: https://doc.rust-lang.org/book/

## Installing rust on mac
- https://www.rust-lang.org/tools/install
- Extension: (.rs)

## First Program



- rustc command is used to compile the code and give you binary.
- Like other languages execution starts from main function.
- A semicolon is needed to end the statements.
- For printing digits we need a placeholder **println!("{}", 2)**, just like python placeholder will take value of digit and treat it like a string. If we pass 2 directly to **println!(2)** then we will get an error on compiling `format argument must be a string literal`
- You can specify any datatype as an argument to placeholder.


## Datatypes in Rust
- Rust is a statically typed language, which means compiler must know datatype of all variables while compilation.
- Rust compiler can automatically infer the type we want to use based on a value.
- We need to define types when compiler doesn't know type of variable (Or where many types are possible) eg: conversion of string to integer.
- Two types are possible: Scalar and compound.

## Variables and datatypes
- Using `let` keyword we create variables.
`fn main() {
    let a = "hello";
    println!("{}", a)
}`
- In above program, `{}` are replaced by a variable.  
`fn main() {
    let a = 1;
    println!("{}", a)
}`
- This will also work perfectly fine. 
- Here rust is automatically infering type of the variable.
- Scalar type represents a single value for eg. 1, 9.4, 'c', true or false.
- Four primary scalar types: integers, floating point numbers, booleans and characters.

### Integer Type
- i8 (signed), u8 (unsigned)
- i16, u16
- i32, u32
- i64, u64
- i128, u128
- isize, usize

`fn main() {
    let a:i32 = 1;
    println!("{}", a)
}`
- Program will compile successfully
`fn main() {
    let a:i32 = 1.1;
    println!("{}", a)
}`
- Will give mismatched type error.
`error[E0308]: mismatched types
 --> src/main.rs:2:17
  |
2 |     let a:i32 = 1.1;
  |           ---   ^^^ expected `i32`, found floating-point number
  |           |
  |           expected due to this
`
- unsigned types cannot be negative otherwise compiler will result in error `unary negation of unsigned integer`
- <Picture of how many numbers can a signed integer can store>
- <Picture of how many numbers can a unsigned integer can store>

### Float types
- only two primitive types f32 and f64.

### Boolean types
- true or false
- bool keyword to use.

### Character type 
- 'c', enclosed in a single quotations.

`fn main() {
    let a:bool = true;
    println!("{}", a);
    
    let b:char = 'c';
    println!("{}", b);
}`

# Mutability
- By default variables are **immutable**
- When a variable is immutable, once a value is bound to a variable, you can't change that value.

		fn main() {
    			let a = 10;
    			println!("{}", a);
    
    			a = 20;
    			println!("{}", a);
    
		}
- Above program will result in error on compilation.

		error[E0384]: cannot assign twice to immutable variable `a`
 		--> src/main.rs:5:5
  		  |
		2 |     let a = 10;
		  |         -
		  |         |
		  |         first assignment to `a`
		  |         help: make this binding mutable: `mut a`
		...
		5 |     a = 20;
		  |     ^^^^^^ cannot assign twice to immutable variable

- While the **Immutability** has its own advantages but mutability also has its advantages in some use-cases.
- We can use `mut` keyword to make one variable as mutable.

		fn main() {
		    let mut a = 10;
		    println!("{}", a);
    
		    a = 20;
		    println!("{}", a);
 		   
		}

## Constants
- They are not immutable variables.
- Contants are values that are bound to a name and are not allowed to change.
- You are not allowed to use `mut` keyword with constants.
- We declare contants with `const` keyword. 
- Type of the value must be annotated. (You need to specify types)
- Constants can be declared in global scope. Using let doesn't allow us to declare variables in global scope.
- Contants cannot take a value returned by a function. It may be set only to a constant expression.
- Constant names are to be written in uppercase (According to rust best practises)
		fn main() {
		    const MAX:i32 = 10;
		    println!("{}", MAX);
		}
- As the name suggests, constant variables cannot be reassigned.
		error: expected one of `.`, `;`, `?`, `}`, or an operator, found `println`
		 --> src/main.rs:5:5
		  |
		4 |     MAX = 20
		  |             - expected one of `.`, `;`, `?`, `}`, or an operator
		5 |     println!("{}", MAX);
		  |     ^^^^^^^ unexpected token
- let cannot be used in global scope. 
		let o = 10
		fn main() {
		    const MAX:i32 = 10;
		    println!("{}", MAX);
		}

Output: 
		error: expected item, found keyword `let`
		 --> src/main.rs:1:1
		  |
		1 | let o = 10
		  | ^^^ expected item

- But const can be used in global scope
		const O:i64 = 40;
		fn main() {
		    const MAX:i32 = 10;
		    println!("{}", MAX);
		    println!("{}", O);
		}

## String
- Something that you write in double quotes is not string in rust. That is str or string slice.
- Rust has only one string type in the core language, which is the string slice (&str).
- The String type, is provided by Rust's standard library rather than coded into core language.
- String is a growable, mutable, UTF 8 encoded type.
		fn main() {
		    let a = "hello"; // type str
		    println!("{}", a);
		}
- To create string, `String::new();` or `String::from("Hello")`. First way will create an empty string while second method will create String from str.
		fn main() {
		    let a = String::new(); // type String
		    println!("{}", a); // empty string
		}
Prog#2:
		fn main() {
		    let a = String::new(); // type String
		    a = "abc";
		    println!("{}", a); // empty string
		}
Output:
		error[E0308]: mismatched types
		 --> src/main.rs:3:9
		  |
		3 |     a = "abc";
		  |         ^^^^^
		  |         |
		  |         expected struct `std::string::String`, found `&str`
		  |         help: try using a conversion method: `"abc".to_string()`
Prog#3:
		fn main() {
		    let a = String::new(); // type String
		    a = String::from("abc");
		    println!("{}", a); // empty string
		}
Output: 
		warning: value assigned to `a` is never read
		 --> src/main.rs:2:9
		  |
		2 |     let a = String::new(); // type String
		  |         ^
		  |
Prog#4:           
		fn main() {
		    let mut a = String::new(); // type String
		    a = String::from("abc");
		    println!("{}", a); // empty string
		}
Output: 
		abc
- Any value in double quotes is of type reference of Str.
- String is growable and extendible.
- `push_str()` is declared only by String type not by str reference or slice of string.
- string slice is immutable you cannot add something to it.

