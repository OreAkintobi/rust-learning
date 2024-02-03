// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    //Function to add 2 numbers
    fn add(a: i32, b: i32) -> () {
        println!("The sum of {:?} and {:?} is {:?}", a, b, a + b);
    }

    add(2, 3);
}
