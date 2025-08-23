// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

pub fn sum_of_two_numbers() -> i32 {
    let number_1 = 5;
    let number_2 = 8;

    return number_1 + number_2;
}

pub fn the_result_of_two_numbers() {
    let result = sum_of_two_numbers();
    println!("{:?}", result);
}

// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

pub fn display_boolean() {
    let display = true;
    if display == true {
        println!("{:?}", "hello");
    } else {
        println!("{:?}", "goodbye")
    }
}

// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

pub fn flow_control() {
    let number = 8;

    if number > 5 {
        println!("{}", "the number is bigger then 5 ")
    } else if number < 5 {
        println!("{}", "the number is smaller then 5 ")
    } else {
        println!("{}", "the number is 5 ")
    }
}
