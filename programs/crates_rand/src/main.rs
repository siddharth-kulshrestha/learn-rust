extern crate rand;
//use rand::Rng;
use rand::{thread_rng, Rng};

fn main() {
    println!("Here are few random numbers!!");

    let mut rng = thread_rng();
    let rns:u32 = rng.gen_range(0..=10);


    println!("{}", rns);
}
