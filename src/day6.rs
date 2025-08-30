// day5

// working with data
//
//
//enum/Enumeration
//
//date that can be one of multiple different possibilities
// each possibility is called a "variant "
// Provides info about your program to compiler
// more robust programs
//

// enum Direction {
//     up,
//     down,
//     Left,
//     Right,
// }

// if want to add a new variant to this direction"forward"
//then this match function will no langer satisfied the all possibilities
// we dont check the this will increase the  reliability of the code no bug will be created because every possibility will be checked

// fn which_way(go: Direction) {
//     match go {
//         Direction::up => "up",
//         Direction::down => "down",
//         Direction::Left => "Left",
//         Direction::Right => "Right",
//     }
// }

// enums can only be one variant at a time
// more robust program when paired wit match function
// make program code easier to read

//Demo /enum

enum Directions {
    Left,
    Right,
    // Up,
}
pub fn which_direction() {
    let go = Directions::Left;
    match go {
        Directions::Left => println!("go Left"),
        Directions::Right => println!("go Right "),
    }
}
//
//
//Activity
// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Green,
    Red,
    Blue,
}

fn print_colors_names(colors: Colors) {
    match colors {
        Colors::Blue => println!("this is blue"),
        Colors::Green => println!("this is green"),
        Colors::Red => println!("this is red"),
    }
}

pub fn main() {
    print_colors_names(Colors::Blue);
    print_colors_names(Colors::Red);
    print_colors_names(Colors::Green);
}
