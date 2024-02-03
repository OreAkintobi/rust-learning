// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinksFlavor {
    Vanilla,
    Strawberry,
    Chocolate,
    Kiwi,
}

struct Drink {
    flavor: DrinksFlavor,
    fluid_ounce: f64,
}

fn print_drink_details(drink: Drink) -> () {
    match drink.flavor {
        DrinksFlavor::Chocolate => println!("The drink flavor is Chocolate"),
        DrinksFlavor::Strawberry => println!("The drink flavor is Strawberry"),
        DrinksFlavor::Vanilla => println!("The drink flavor is Vanilla"),
        DrinksFlavor::Kiwi => println!("The drink flavor is Kiwi"),
    }
    println!("The drink weighs {:?} ounces", drink.fluid_ounce);
}

fn main() {
    let drink1 = Drink {
        flavor: DrinksFlavor::Vanilla,
        fluid_ounce: 3.20,
    };

    let drink2 = Drink {
        flavor: DrinksFlavor::Chocolate,
        fluid_ounce: 3.20,
    };

    let drink3 = Drink {
        flavor: DrinksFlavor::Strawberry,
        fluid_ounce: 3.20,
    };

    let drink4 = Drink {
        flavor: DrinksFlavor::Kiwi,
        fluid_ounce: 3.20,
    };

    print_drink_details(drink1);
    print_drink_details(drink2);
    print_drink_details(drink3);
    print_drink_details(drink4);
}
