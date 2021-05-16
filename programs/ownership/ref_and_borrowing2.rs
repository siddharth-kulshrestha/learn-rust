// Borrowing:
// References as function parameters is known as borrowing. 
// for ex: 
fn will_not_take_ownership(s1: &String) {
    println!("s1: {}", s1);

}
// 
// If we try to modify or mutate some value we are borrowing, then we will get an error. 
// But by adding mut keyword we can allow borrower to modify but WITH SOME RULES.
//
//
fn borrow_with_mutate(s1: &mut String) {
    s1.push_str(", World!!!!!!!!");
    println!("s1: {}", s1);

}

// Rules of references:
// 1. You can create any number of immutable references.
// 2. You can only one mutable reference in a scope.
// 3. You cannot create mutable reference if your program uses more than one immutable references.


fn main() {
    let s1 = String::from("Hello");
    let mut s2 = String::from("Hello 2");

    will_not_take_ownership(&s1);
    will_not_take_ownership(&s2);
    borrow_with_mutate(&mut s2);


    let r1 = &s1;
    let r2 = &s2;
    let r3 = &s2; // Immutable reference of mutable string
    let r4 = &s1; // Immutable reference of immutable string

    println!("{}, {}, {} and {}", r1, r2, r3, r4);

    //let mr1 = &mut s1; // Mutable reference of immutable string // Not allowed you can only

    //create mutable reference of mutable values.
    let mr2 = &mut s2; // Mutable reference of mutable string
    //let mr3 = &mut s2; // RULE NUMBER 2: Will not be allowed as you can only have one mutable reference in one scope.
    //let mr4 = &mut s2; // RULE NUMBER 2: Will not be allowed as you can only have one mutable reference in one scope.

    //println!("{}, {} and {}", mr2, mr3, mr4);
    println!("{}", mr2); // Now 

    //println!("{}, {}, {} and {}", r1, r2, r3, r4); // Error since we have created a mutable reference
    //so all previous immutable references will be invalidated. Check RULE 3
    

    let r5 = &s2;
    println!("{}", r5);
    //println!("{}", mr2); // Error since we have created immutable reference r5 so all previous mutable references will become invalid.

    // Creating new scope for using another mutable reference
    //
    {
        let mr3 = &mut s2; 
        println!("{}", mr3);
    } // Cannot use mr3 here.



}
