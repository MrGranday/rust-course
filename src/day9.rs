//Data structure
//*Vector */
//
// it is multiple pieces of data
// it must be the same type
// used for lists of information
// can add , remove , and traverse the entries

//*(Example) */
pub fn vec_new() {
    //*we have declared a vector */
    let my_vec = vec![1, 2, 3];

    //*we have created a vector */
    let mut my_vec = Vec::new();

    //*we have pushed values  */
    println!("{:?}", my_vec.push(1));
    println!("{:?}", my_vec.push(3));

    //*we have pop the values */
    println!("{:?}", my_vec.pop());

    //*the length of the vector */
    println!("{:?}", my_vec.len());
}
