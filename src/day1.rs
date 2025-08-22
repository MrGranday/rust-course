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

// Note there are Singed and Unsinged

//i8 and u8
//Size in (bytes) are byte 1
//i8 ---- min -128   max 127
//u8-----min 0     max 255
// and we have ( i16-u16  i32-u32  i64 u64  i128  u128  isize  usize)
//isize and usize it is depending on the system

// the println macro
// println!(add(4, 5));

fn this_for_print_macro() {
    let life = 42;
    println!("hello");
    println!("{}", life);
    // IF WE see we can print this variable in other two ways also
    // this is the debug version of the println
    println!("{life :?}{}", "this is the debug version");

    // we can use the end user display version also
    println!("{life} {}", " this is the end user display version");
}
