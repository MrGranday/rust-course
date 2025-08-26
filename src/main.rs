// for using another file in the main file to execute we use "mod" and the name of the file
// the functions that u are using it should be public it means it should be "pub"
// if you do not write "pub" in the begging of the function start it will be privet
//e.g.

//private function
// fn () {

// }

//public function
// pub fn () {

// }

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    //DAY 1
    println!("start of DAY1");
    //this is for add function
    day1::add(3, 4);
    // println!("{}", result);

    //this is for macro print
    day1::this_for_print_macro();

    //DAY2
    println!("start of DAY2");
    //for the control flow
    day2::control_flow();
    println!(" {}", "this is for the control flow ");

    // for infinite loop
    day2::infinite_loop();
    println!(" {}", "this is for the infinite loop");

    //for conditional loop
    day2::conditional_loop();
    println!("this is the conditional loop");

    day2::first_name();
    day2::last_name();

    //DAY3
    println!("start of DAY3");
    // for the sum of two numbers
    day3::the_result_of_two_numbers();
    day3::display_boolean();
    day3::flow_control();
    //we give the values for both value_1 and value_2 in this
    let the_result = day3::arithmetic(4, 5);
    println!("{:?}", the_result);

    //DAY4
    println!("start of DAY4");
    // match expression
    day4::match_exp();
    day4::match_int();
}
