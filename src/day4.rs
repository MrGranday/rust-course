//DAY4
//Match
// thi expression add logic to program
// these are similar to if else
// exhaustive
/// aff option must be accounted for

pub fn match_exp() {
    let boole = true;

    match boole {
        true => println!("this is true"),
        false => println!("this is false "),
    }
}

// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

pub fn match_int() {
    let some_int = 3;
    match some_int {
        1 => print!("this is one "),
        2 => print!("this is two "),
        3 => print!("this is three"),
        _ => print!("it another number "),
    }
}
