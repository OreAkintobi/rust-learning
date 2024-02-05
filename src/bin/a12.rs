// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Brown,
    Other,
}

impl BoxColor {
    fn print(&self) -> () {
        match self {
            BoxColor::Brown => println!("This box is brown in color"),
            BoxColor::Other => println!("This box is some other color"),
        }
    }
}

struct BoxDimensions {
    height: f64,
    width: f64,
    depth: f64,
}

impl BoxDimensions {
    fn print(&self) -> () {
        println!(
            "This box has {:?} inches height, {:?} inches width, and {:?} inches depth",
            self.height, self.width, self.depth
        );
    }
}

struct ShippingBox {
    dimensions: BoxDimensions,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn new(dimensions: BoxDimensions, weight: f64, color: BoxColor) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) -> () {
        self.color.print();
        self.dimensions.print();
        println!("This box weighs {:?} kg", self.weight)
    }
}

fn main() {
    let dimensions_1 = BoxDimensions {
        height: 2.4,
        width: 3.5,
        depth: 4.2,
    };

    let dimensions_2 = BoxDimensions {
        height: 5.4,
        width: 7.5,
        depth: 9.2,
    };

    let shipping_box_1 = ShippingBox::new(dimensions_1, 2.25, BoxColor::Brown);
    let shipping_box_2 = ShippingBox::new(dimensions_2, 3.35, BoxColor::Other);

    shipping_box_1.print();
    shipping_box_2.print();
}
