// Match example
//
// match is a replacement for switch, but is powerful than switch. 
// A match works on integers, ranges of integers, bools, enums, tuples, arrays and structs.
// It will destructure tuples, arrays and structs.
// It requires a default handler if necessary. 
//
//   
//
//


fn match_example_one(choice: i32) {
    println!("Choice variable: {}", choice);
    match choice {
        1 => println!("This is one!"),
        2 => {
            println!("This is two ---> First!");
            println!("This is two --> Second!");
        },
        13 => println!("Number is 13"),
        6 | 7  | 8 | 9 => {
            println!("This number is either 6, 7, 8 or 9!!")
        },
        100..=200 => println!("this number is between 100 to 200, both inclusive."), 
        _ => {
            println!("This will match any pattern which is not 1,2 or 13");
        },
    }
}

fn match_example_enums(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Coin is: {:?}", coin);
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn match_example_enums_with_value(coin: CoinInStates) -> u32 {
    match coin {
        CoinInStates::Penny => {
            println!("CoinInStates is: {:?}", coin);
            1
        },
        CoinInStates::Nickel => 5,
        CoinInStates::Dime => 10,
        CoinInStates::Quarter(state) => {
            println!("State: {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum CoinInStates {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
    //etc
}


fn main() {

     // Example 1: Match with numbers
    let option_number = 13; 
    match_example_one(option_number);
    match_example_one(19);
    match_example_one(2);
    match_example_one(1);
    match_example_one(6);
    match_example_one(7);
    match_example_one(100);
    match_example_one(200);
    match_example_one(1200);

    // Example 2: Match of enums
    println!("Penny: {}", match_example_enums(Coin::Penny));
    println!("Nickel: {}", match_example_enums(Coin::Nickel));
    println!("Dime: {}", match_example_enums(Coin::Dime));
    println!("Quarter: {}", match_example_enums(Coin::Quarter));

   // Example 3: Encapsulating value inside enum variant
   println!("Quarter: {}", match_example_enums_with_value(CoinInStates::Quarter(UsState::Alaska)));
   println!("Quarter: {}", match_example_enums_with_value(CoinInStates::Quarter(UsState::Arizona)));

}
