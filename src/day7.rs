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
fn working_with_tuples() {
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

// activity
// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn tuple_cartesion() -> (i32, i32) {
    (4, 5)
}
pub fn coord() {
    let (x, y) = tuple_cartesion();
    if y > 5 {
        println!("this is >5");
    } else if y < 5 {
        println!("this is <5");
    } else {
        println!("this is =5");
    }
}

//Expressions
//
//
// Rust is an expression-based language
//must things are evaluated and return some value
//expression values coalesce to a single point
//can be used for nesting logic
pub fn my_num_expression() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 { true } else { false };

    // we can achieve the same result without using the if else expression in here
    let this_is_second = my_num < 5;

    // these both are the same the above is using the if else statement to get the result if it is true or false
    // but the seconde on is just using the variable and comparing it and storing the true or false
    println!("{:?}{:?}", is_lt_5, this_is_second);
    // in here the result will be true  in both version the result will be true
    println!("we can do the same with match expressions ");

    let my_num2 = 3;
    let message = match my_num2 {
        1 => "hello",
        _ => "hi",
    };

    println!("{}{:?}", "this is for the match expression: ", message);

    // it is possible to nest the expression as many times

    //for example

    println!("using with the enum ");
    //
    enum Menu {
        Burger,
        Drinks,
        Fries,
    }

    let paid = true;
    let item = Menu::Drinks;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drinks => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };
    if paid == true {
        println!("he paid: ")
    } else {
        println!(" he did pay: ")
    };
    println!("{}{:?}", "yes i what to buy this: ", order_placed,);
}

//
//
// Expressions allow nested logic
//if and match expression can be nested
//best to not use more then two or three levels

//Demo
enum Access {
    Admin,
    Manager,
    User,
    Guest,
}
pub fn the_level_system() {
    // secret file: admin only
    //the use access level is determines in here
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("{}{:?}", " the access level is : ", can_access_file);
}

// Activity

// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
pub fn this_big_num() {
    let value = 100;
    let result = if value > 100 { true } else { false };

    match result {
        true => println!("it small: "),
        _ => println!("it big: "),
    };
}
