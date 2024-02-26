// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

/// Struct to store at least the age of a customer
struct Customer {
    name: String,
    age: i32,
}

/// Determine if a customer can make a purchase
fn can_make_purchase(customer: &Customer) -> Result<bool, String> {
    if customer.age >= 21 {
        Ok(true)
    } else {
        Err("Customer is not old enough".to_string())
    }

    // alternative to the above if-else block
    // match customer.age {
    // 	age if age >= 21 => Ok(true),
    // 	_ => Err("Customer is not old enough".to_string())
    // }
}

fn main() {
    let customer = Customer {
        name: "Bubba".to_owned(),
        age: 20,
    };

    match can_make_purchase(&customer) {
        Ok(true) => println!("{:?} can make purchase", customer.name),
        Ok(false) => println!("{:?} cannot make purchase", customer.name),
        Err(e) => println!("{:?} cannot make purchase: {}", customer.name, e),
    }
}
