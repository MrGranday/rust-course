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

//* fn display_light(light: Light) {
// *    match light {
// *       Light::Bright => println!("bright"),
// *       Light::Dull => println!("dull"),
// *      }
// *}

// *fn main() {
// *    let dull = Light::Dull;
// *    display_light(dull);
// *    display_light(dull);
// * }
