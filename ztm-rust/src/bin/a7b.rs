// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//


// Enums can only be one variant at a time, more robust programs are paired with match and makes code easier to read
// we use the :: (double semi colon) to assiant the varient to the enumeration
// when you make an enum you are making your own custom type which can be used in args and type hinting

// * Use an enum with color names as variants
enum Color {
    Red,
    Yellow,
    Blue,
}

// * Use a function to print the color name, The function must use the enum as a parameter, Use a match expression to determine which color
fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
    }
}

fn main() {
    print_color(Color::Blue);
}
