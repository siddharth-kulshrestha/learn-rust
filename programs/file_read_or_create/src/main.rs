// Lets understand the usage of result enum, having variants Ok and Err.
//
// We will read a file if that file is not present we will create a file by the same name.
//
//

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn read_or_create(filename: &str) -> File {
    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("Trying to create a new file!");
            match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Not able to create file {:?}", e)
                },
            }
        },
        Err(error) => panic!("Unable to open file {:?}", error), 
   };
   f
}

fn unwrap_and_expect() {

    // unwrap syntax
    // it throws panic automatically if result is error
    let f = File::open("hello.txt").unwrap();
    println!("{:?}", f);
    //let f = File::open("hello2.txt").unwrap();
    //println!("{:?}", f);

    // Using expect we can create our own error messages
    let f = File::open("hello.txt").expect("Failed");
    println!("{:?}", f);
    //println!("Should fail:");
    //let f = File::open("hello2.txt").expect("Failed");
    //println!("{:?}", f);

// Choosing between unwrap and expect is our choice, we can choose unwrap where we dont want error
// message of our own choice and we can choose expect where we want error message of our own.
}

fn read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    // Returning result
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_with_ques_mark_operator() -> Result<String, io::Error> {
    
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = read_or_create("hello.txt");
    println!("{:?}", f);

    unwrap_and_expect();

    // Propagating errors
    // Lets read a file and send error to this main function.
    let op = read_file();
    match op {
        Ok(fi) => println!("{:?}", fi),
        Err(e) => println!("{:?}", e),
    }
    // As you can notice propagating errors in rust is a tedious task so we would want to simplify
    // it and rust provides `?` operator to simplify this.
    println!("Read with question mark operator:::::\n\n");
    let op = read_with_ques_mark_operator();
    match op {
        Ok(fi) => println!("{:?}", fi),
        Err(e) => println!("{:?}", e),
    }

    
    println!("Hello, world!");
}

