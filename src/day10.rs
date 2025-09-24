//*Activity */
// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String

struct Info {
    name: String,
    color: String,
    age: i32,
}
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

pub fn print_info() {
    let vectors = vec![
        Info {
            name: "osman".to_owned(),
            color: "yellow".to_owned(),
            age: 10,
        },
        Info {
            name: "nasir".to_owned(),
            color: "blue".to_owned(),
            age: 10,
        },
        Info {
            name: "omer".to_owned(),
            color: "green".to_owned(),
            age: 9,
        },
        Info {
            name: "abid".to_owned(),
            color: "red".to_owned(),
            age: 8,
        },
    ];

    for person in vectors {
        if person.age < 10 {
            println!(
                "this is under 10 years---> name: {:?}, color: {:?}, age: {:?}",
                person.name, person.color, person.age
            )
        } else {
            println!(
                "these are 10 years old----> name: {:?}, color: {:?}, age: {:?}",
                person.name, person.color, person.age
            )
        }
    }
}
