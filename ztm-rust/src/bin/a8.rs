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

// A struct is a type which contains multple pieces of data, all or nothing
// each piece of data is called a field, similar data can be grouped together
// fields within a struct can be accessed using a dot (.)

enum Flavour {
    Sparking,
    Sweet, 
    Fruity
}

struct Drink {
    flavour: Flavour,
    fluid_oz: f64,
}

fn print_drink (drink: Drink) {
    match drink.flavour {
        Flavour::Sparking => println!("flavour: sparking"),
        Flavour::Sweet => println!("flavour: sweet"),
        Flavour::Fruity => println!("flavour: fruity"),
    }

    println!("OZ: {:?}", drink.fluid_oz);
}
fn main() {
    let sweet = Drink {
        flavour: Flavour::Sweet,
        fluid_oz: 6.0
    };
    print_drink(sweet);
    let fruity = Drink {
        flavour: Flavour::Fruity,
        fluid_oz: 7.0,
    };
    print_drink(fruity);
}
