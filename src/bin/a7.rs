// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#[derive(Debug)]
enum Color {
    Black,
    White,
    Red,
    Yellow,
    Blue,
}

fn print_color(color: Color) -> () {
    match color {
        Color::Black => println!("Black"),
        Color::White => println!("White"),
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        _ => println!("{:?}", color),
    }
}

fn main() {
    print_color(Color::Blue);
    print_color(Color::Black);
    print_color(Color::Red);
    print_color(Color::White);
    print_color(Color::Yellow);
}
