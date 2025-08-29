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
// using while loop

pub fn le_loop() {
    // creating mut variable that i would change the value later
    let mut value = 1;
    while value <= 3 {
        println!("{:?}", value);
        value = value + 1;
    }
}

// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

pub fn while_loop() {
    println!("while loop");
    println!("       ");

    let mut i = 5;

    while i >= 1 {
        println!("{:?}", i);
        i = i - 1;
        if i == 1 {
            println!("done");
        }
    }
}
