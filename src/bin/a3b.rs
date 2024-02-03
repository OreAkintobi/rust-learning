// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn more_or_less_than_five(num: i32) -> () {
    let compare_number = 5;

    if num < compare_number {
        println!("<5")
    } else if num > compare_number {
        println!(">5")
    } else {
        println!("=5")
    }
}

fn main() {
    let a = 6;
    let b = 2;
    let c = 5;

    more_or_less_than_five(a);
    more_or_less_than_five(b);
    more_or_less_than_five(c);
}
