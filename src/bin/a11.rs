// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_grocery_quantity(item: &GroceryItem) -> () {
    println!("There are {:?} items", item.quantity)
}

fn display_grocery_id(item: &GroceryItem) -> () {
    println!("Item id is {:?}", item.id_number)
}

fn main() {
    let grocery = GroceryItem {
        quantity: 12,
        id_number: 12021,
    };

    display_grocery_quantity(&grocery);
    display_grocery_id(&grocery);
}
