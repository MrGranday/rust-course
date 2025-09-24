//Data structure
//*Vector */
//
// it is multiple pieces of data
// it must be the same type
// used for lists of information
// can add , remove , and traverse the entries

//

//*(Example) */
pub fn vec_new() {
    //*we have CREATED a vector with the data you know to push  */
    let mut my_vec = vec![1, 2, 3];

    //*we have CREATE a NEW EMPTY vector that has [0] length nothing inside but it will give you error saying[Vec{unknown}] */
    //*so we must push some data to it that it know the data tpe if string or number */
    let mut my_vec2 = Vec::new();
    my_vec2.push(1);

    //*but we can CREATE a NEW EMPTY vector in this way also  that has [0] length nothing inside but it will give you error saying[Vec{unknown}]*/
    //*so we must push some data to it that it know the data tpe if string or number */
    let mut my_vec1 = vec![];
    my_vec1.push(1);

    //*and if you create it like this and specify the the type of VECTOR then even if it is empty it will not give you error */
    //*but it will give you warnings about that this vector is empty */
    let mut my_vec3: Vec<i32> = vec![];
    let mut my_vec4: Vec<i32> = Vec::new();

    //*we have pushed values  */
    my_vec.push(1);
    my_vec.push(3);

    //*we have pop the values */
    println!("the last item we pop or eliminate:  {:?}", my_vec.pop());

    //*the length of the vector */
    println!("this is the length of my_vec :{:?}", my_vec.len());

    //*now iterating over the vector  */
    for num in my_vec {
        println!(" printing all the data in the vector one by one :{:?}", num)
    }
}

//
//*Demo */
struct Test {
    score: i32,
}

pub fn the_score() {
    let my_score = vec![
        Test { score: 91 },
        Test { score: 98 },
        Test { score: 45 },
        Test { score: 65 },
        Test { score: 45 },
    ];
    for test in my_score {
        println!("the score are : {:?}", test.score);
    }
}

// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

pub fn working_on_vec() {
    let my_vec = vec![10, 20, 31, 30, 40];
    println!("this loop: ");
    for num in &my_vec {
        if num == &30 {
            println!("{:?}", "Thirty");
        } else {
            println!("{:?}", num)
        }
    }
    println!("the number of elements :{:?}", my_vec.len())
}

//Data Type
//*Strings */
//
//
// two commonly used types of strings
//String - owned
//&str - borrowed String slice
// we must use an owned (String) to store in a struct
// using &str when [passing] to a function

fn print_string(the_data: &str) {
    println!("{:?}", the_data)
}

pub fn print_the_string() {
    //*one way to do it  */
    print_string("this is one way to do it: ");

    //* another way to do it */
    let with_to_owned_fn = "this is another way to do it".to_owned();

    //*another way to do do is using String from */
    let with_string_from = String::from("this way also ");

    //*then we can call it in the function  */
    print_string(&with_string_from);
    print_string(&with_to_owned_fn);
}

//if want to use with struct
struct Employee {
    name: String,
    //*we use String not &str in the struct */
}

pub fn print_the_employee() {
    //*we can do this also and another way */
    let emp_name = "osman".to_owned();

    //*in this way also */
    let emp_name1 = String::from("osman");

    let _emp = Employee { name: emp_name };
}

// *Strings are automatically borrowed*/
//*Use .to_owned() or String::from() to create an owned copy of a string slice  */
//*Use an owned String when storing in a struct */
//
//
//
//*Demo */
struct LineItem {
    name: String,
    count: i32,
}

pub fn the_line_item() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 2,
        },
    ];

    for items in receipt {
        println!("name: {:?},  count: {:?}", items.name, items.count);
    }
}
