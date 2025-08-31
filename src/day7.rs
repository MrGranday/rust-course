//CONTINUING WORKING WITH DATA
//Tuples
//
// it is a type of "recode"
// store data anonymously
// no need to name fields
//useful to return pairs of data from functions
// can be "destructured" easily into variables
//
//
//

// enum Access {
//     Full,
// }
//*we are retuning three separate values within the tuple  */
//* we have i32 integer it is tuple because it is surrounded   with the brackets with three values */
// fn one_two_three() -> (i32, i32, i32) {
//     (1, 2, 3)
// }
//*this tuple will be populate with the numbers variable  */
// let numbers= one_two_three();

//*this is called destructuring (1,2,3) will be populated with (x,y,z) */
// let (x,y,z)= one_two_three();
//*this 0,1,2 is the index of the tuple we are accessing the tuple via index and printing it out  */
// println!("{:?},{:?}", x , numbers.0);
// println!("{:?},{:?}", y , numbers.1);
// println!("{:?},{:?}", z , numbers.2);

//*in here we are mixing to types of data when we are working with tuples   */
// let=(employee, access)=("Jake",Access::Full)
//
//
// tuple allow for anonymous data access
// tuple is useful when destructuring
//tuples can contain any number if fields
// use struct when more then 2 or 3 fields
//
//Demo
pub fn working_with_tuples() {
    let coordinates = (2, 3);
    println!("{:?},{:?}", coordinates.0, coordinates.1);

    let (x, y) = (2, 3);
    println!("{:?},{:?}", x, y);

    //*the best practice is to name every variable that can be better for understanding  */
    let (name, age) = ("osman ", 20);

    //*it shouldnt be like this  */
    let favorites = ("red", 2, "kabul", "pizza", "park");
    let city = favorites.2;
    let place = favorites.4;
}
