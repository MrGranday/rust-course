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
