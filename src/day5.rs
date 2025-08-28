// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

pub fn print_on_no() {
    let mut number = 1;
    loop {
        println!("{}", number);
        println!("the number print");
        number += 1;
        if number > 4 {
            break;
        }
    }
}
