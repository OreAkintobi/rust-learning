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

fn boolean(num: i32) -> bool {
    if num > 100 {
        return true;
    } else {
        return false;
    }
}

fn print_message(condition: bool) -> () {
    match condition {
        true => println!("its big"),
        _ => println!("its small"),
    }
}

fn main() {
    let num = 100;
    let vl_gt_100 = num > 100;

    print_message(boolean(num));
    print_message(vl_gt_100);
}
