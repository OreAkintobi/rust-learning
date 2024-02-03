// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn is_true_or_false(bool: bool) -> () {
    match bool {
        true => print!("it's true"),
        false => print!("it's false"),
    }
}

fn main() {
    let a = true;
    let b = false;

    is_true_or_false(a);
    is_true_or_false(b);
}
