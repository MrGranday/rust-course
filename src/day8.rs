//Day8
//
// *Basic memory refresh*/
//memory is stored using binary // Bits: 0 or 1
//Computer optimized for bytes // 1 byte == 8 contiguous bits
//fully contiguous

//
//*Addresses*/
// all data in the memory has an "address"
//used to locate data
//always the same - only data changes
//usually don't utilize addresses directly
// variable handle most of the work

//
//
//*Offset8*/
//Items can be located at an address using an "offset"
//Offsets begin at 0
// Represent the number of bytes away from the original address
//normally deal with indexes instead

//memory uses addresses and offsets
//addresses are permanent, data differs
//offsets can be used to "index" into some data

//*Ownership */
//
//Programs must track memory
//if they fail to do so a leak occurs
//Rust utilizes an "ownership" model to manage memory
//the "owner"of memory is responsible for cleaning up the memory
// memory can either be "moved" or "borrowed"

//*Example of - Move */
//
//

// *enum Light {
//*      Bright,
// *    Dull,
// *}
// 3 when it is moved to this function now it is owned by this function
// 4 when it is here and the functions is finished executing this "light" and then the "the light will be deleted "
//* fn display_light(light: Light) {
// *    match light {
// *       Light::Bright => println!("bright"),
// *       Light::Dull => println!("dull"),
// *      }
// *}

// 1 first the dull light is owned by this function
// *fn main() {
// 5 now this "let dull " is not available for the next time executing because the new owner
//display_light delete this when it was finished executing this
// *    let dull = Light::Dull;
// 2 then it moved the dull to display_light function and the ownership is moved
// *    display_light(dull);
// 6 so there for the next time this will not run it will give you error
// *    display_light(dull);
// * }

// For this we have solution we use Borrowing

//*Example of - Borrow */
//
//
// her we use "&" it means borrowing data or referencing data

// *enum Light {
//*      Bright,
// *    Dull,
// *}
// 3 when it is borrowed to this function the  owner is still the main function
// 4 it will one time execute it by borrowing
//* fn display_light(light: &Light) {
// *    match light {
// *       Light::Bright => println!("bright"),
// *       Light::Dull => println!("dull"),
// *      }
// *}

// 1 first the dull light is owned by this function
// *fn main() {
// 5 now this "let dull " is  available for the next time executing because the owner
//is still the main function and it was borrowed
// *    let dull = Light::Dull;
// 2 then it moved the dull to display_light function and the ownership is moved
// *    display_light(&dull);
// 6 so there for the next time this will run it will not give you error because it was borrowed
// 7 so the dull was not deleted because the main function was not fully execute
// when it goes to the end and the main function is finished then it will delete the dull
// *    display_light(&dull);

// * }

//
//
//*Recap */
// Memory must be managed in some way to prevent leaks
//Rust uses "Ownership " to accomplish memory management
//the "Owner" of data must clean uo the memory
//this occurs automatically at the end of the scope
//Default behavior is to "move" memory to new owner
//Use an ampersand(&) to allow code to "borrow " memory

//*Demo */
struct Book {
    rating: i32,
    pages: i32,
}

fn display_rating(book: &Book) {
    println!("the rating is = {:?}", book.rating);
}

fn display_pages(book: &Book) {
    println!("the pages are = {:?}", book.pages)
}

pub fn to_display_both() {
    // if dont use borrowing it will give us compiler error
    // to use the (book) variable for the second time we borrow it
    // so the ownership dont transfer from (to display_both)
    let book = Book {
        pages: 100,
        rating: 5,
    };
    display_pages(&book);
    display_rating(&book);
}
//
//

//*Activity */
// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id_number: i32,
    quantity: i32,
}

fn display_id_number(grocery: &Grocery) {
    println!("the id number is = {:?}", grocery.id_number);
}

fn display_quantity(grocery: &Grocery) {
    println!("the quantity is = {:?}", grocery.quantity);
}
pub fn to_display_quantity() {
    let grocery = Grocery {
        id_number: 1232,
        quantity: 2,
    };

    display_id_number(&grocery);
    display_quantity(&grocery);
}

//Demo
//*impl */
// it means implementation
// Define a struct (a box of data) that stores a temperature value
struct Temperature {
    degree: f32, // one field: degree (floating point number)
}

// Implementation block = add functions to Temperature
impl Temperature {
    // This is an "associated function"
    // It does not need self because it makes a NEW Temperature
    // "Self" (big S) means "Temperature type"
    //freezing is a constructor (it makes a new Temperature). That’s why it uses Self.
    fn freezing() -> Self {
        // Return a new Temperature object with degree = 10.8
        Self { degree: 10.8 }
        // Same as: fn freezing()->Temperature
        // Temperature { degree: 10.8 }
    }

    // This is a "method"
    // It works on an existing Temperature object
    // "&self" means we borrow the object so we can read it
    fn display_temp(&self) {
        // "self.degree" means: get the degree value of this object
        println!("this is {}", self.degree);
    }
}

// A public function to test everything
pub fn to_display_f() {
    // Make a new Temperature object with degree = 88.8
    let temp = Temperature { degree: 88.8 };

    // Two ways to call the display_temp method:

    // 1) Explicit way: we pass a reference (&temp) ourselves
    Temperature::display_temp(&temp);
    //* */ Both methods work perfectly*/
    // 2) Shorthand way: just call it with dot notation
    // Rust automatically passes "&temp" here
    //* */ Both methods work perfectly*/
    temp.display_temp();

    // Now use the associated function "freezing" to make a cold Temperature
    let cold = Temperature::freezing();

    // Call display_temp on that object too
    cold.display_temp();
}

//*Activity */
// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
#[derive(Debug)] // <- this allows printing BoxColor with {:?}
enum BoxColor {
    Red,
    Blue,
    Black,
    Yellow,
}

#[derive(Debug)] // <- this allows printing ShippingBox with {:?}
struct ShippingBox {
    dimension_x: i32,
    dimension_y: i32,
    weight: i32,
    color: BoxColor,
}

impl ShippingBox {
    fn create_black_box() -> Self {
        Self {
            dimension_x: 3,
            dimension_y: 4,
            weight: 50,
            color: BoxColor::Black,
        }
    }

    fn display_created_box(&self) {
        println!("this box is {:?}", self);
    }
}

pub fn display_the_boxes() {
    let black_box = ShippingBox::create_black_box();
    black_box.display_created_box();
}
