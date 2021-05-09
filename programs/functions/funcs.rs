
// Naming conventions: my_functions(), use fn keyword for writing functions
// functions can be defined anywhere in the program, its not important for callee to be defined
// above caller.
// fn fun_name(arguments_list) { // statements }
// fn fun_name(arguments_list) -> return_type { // statements }
// fn fun_name(arguments_list) -> (return_type1, return_type2) { // statements }
// fn fun_name() {
//      fn fun_2() {
//               // statements
//      }
// }
//


fn main() {
    println!("{:?}", sub_add(2,3));
    // :? is used to print more than one arguments returned by funcs, list types
    

    println!("{}", add(1,2));
    fn add(c:i32, b:i32) -> i32 {
        return c+b;
    }


}



fn sub_add(a:i32, b:i32) -> (i32, i32) {
    //return a+b, a-b;
    
    // you can also ignore return keyword
    (a+b, a-b)
}
