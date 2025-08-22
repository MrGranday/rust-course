mod day1;
mod day2;

fn main() {
    //DAY 1
    //this is for add function
    day1::add(3, 4);
    // println!("{}", result);

    //this is for macro print
    day1::this_for_print_macro();

    //DAY2
    //for the control flow
    day2::control_flow();
    println!(" {}", "this is for the control flow ");

    // for infinite loop
    day2::infinite_loop();
    println!(" {}", "this is for the infinite loop");

    //for conditional loop
    day2::conditional_loop();
    println!("this is the conditional loop");
}
