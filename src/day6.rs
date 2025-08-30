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
}
pub fn which_direction() {
    let go = Directions::Left;
    match go {
        Directions::Left => println!("go Left"),
        Directions::Right => println!("go Right "),
    }
}
