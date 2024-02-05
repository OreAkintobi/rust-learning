// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

struct JustXAndY {
    x: i32,
    y: i32,
}

fn get_coords(coords: JustXAndY) -> (i32, i32) {
    (coords.x, coords.y)
}

fn cartesian_coordinates(coord: JustXAndY, compare_number: i32) -> () {
    let (x, y) = get_coords(coord);

    print!("x is {:?}, ", x);

    if y < compare_number {
        println!("y is less than {:?}", compare_number)
    } else if y > compare_number {
        println!("y is greater than {:?}", compare_number)
    } else {
        println!("y is equal to {:?}", compare_number)
    }
}

fn main() {
    let just_x_and_y = JustXAndY { x: 3, y: 8 };

    cartesian_coordinates(just_x_and_y, 19);
}
