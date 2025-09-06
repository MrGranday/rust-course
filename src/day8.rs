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
