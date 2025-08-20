// DAY 1
// High-level language features without performance penalties
//program behaviors can be enforced
// Rust is First-class multithreading
//compiler error to improperly access shared data
// Type System
//can uncover bugs at the compile time
//make refactoring simple
//reduce the number of tests needed

//Module system make code separation simple
//adding a dependency is 1 line in a config file
// Tooling
//generate doc , lint code , auto format

// DATA TYPES
//BOOLEAN
//true, false
// Integer
//1,2,3,4,5,5
//double , float
// character
// string

// Variables
// Assign data to temporary memory location
// Allows programmes to easily work with memory
//Can be set to any value and type
//Immutable  by default, but can be mutable
// Immutable: cant be changed
//Mutable: can be change

// let two = 2;
// let  hello ="hello";
// let  j ='j';
// let  the_half_number= 0.5;
// let  mut my_name_is ="osman ghani granday";
// let  boolean= false;

//function
//can return and accept data
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = add(3, 4);

    print!("{}", result);
}
