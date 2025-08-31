// day5

// WORKING WITH DATA
//
//
//enum/Enumeration
//
//date that can be one of multiple different possibilities
// each possibility is called a "variant "
// Provides info about your program to compiler
// more robust programs
//

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// if want to add a new variant to this direction"forward"
//then this match function will no langer satisfied the all possibilities
// we dont check the this will increase the  reliability of the code no bug will be created because every possibility will be checked

fn which_way() {
    let way = Direction::Down;
    match way {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    };
}

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

//
//
//
// struct
//
//
// A type that contains multiple pieces of the data
// all or nothing -cannot have some pieces of data and not others
// Each piece of data is called a 'field '
// make working with data easier
// similar data can be grouped together
//
//
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

pub fn the_shipping() {
    let my_box = ShippingBox {
        depth: 4,
        width: 5,
        height: 6,
    };

    println!("the box is {} units tall", my_box.height);
    println!("the box is {} units width", my_box.width);
    println!("the box is {} units depth", my_box.depth);
}
//
//
//structs deals with multiple pieces of data
//all fields must be present to create a struct
//fields can be accessed using a dot(.)
