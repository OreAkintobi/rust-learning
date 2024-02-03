// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// Function to display first name
fn display_first_name(f_name: &str) -> () {
    println!("My First Name is => {:?}", f_name)
}

// Function to display last name
fn display_last_name(l_name: &str) -> () {
    println!("My Last Name is => {:?}", l_name)
}

fn main() {
    display_first_name("Oreoluwa");
    display_last_name("Akintobi");
}
