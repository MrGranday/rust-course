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
