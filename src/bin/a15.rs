// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    // * Create one of each ticket and place into a vector

    let ticket_1 = Ticket::Backstage(15.99, "David".to_owned());
    let ticket_2 = Ticket::Vip(11.99, "Robert".to_owned());
    let ticket_3 = Ticket::Standard(8.99);

    let tickets = vec![ticket_1, ticket_2, ticket_3];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!(
                "This is a Backstage ticket belonging to {:?} with a price of {:?}",
                holder, price
            ),
            Ticket::Vip(price, holder) => println!(
                "This is a Vip ticket belonging to {:?} with a price of {:?}",
                holder, price
            ),
            Ticket::Standard(price) => {
                println!("This is a Standard ticket with a price of {:?}", price)
            }
        }
    }
}
