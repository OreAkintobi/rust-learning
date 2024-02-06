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

fn main() {
    // let mut my_numbers = Vec::new();
    let my_numbers = vec![10, 20, 30, 40];

    // my_numbers.push(10);
    // my_numbers.push(20);
    // my_numbers.push(30);
    // my_numbers.push(40);

    for &num in &my_numbers {
        if num == 30 {
            println!("thirty")
        } else {
            println!("{:?}", num)
        }
    }

    println!("The length of the vector is {:?}", my_numbers.len())
}
