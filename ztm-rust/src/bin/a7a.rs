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

// Enums can only be one variant at a time, more robust programs are paired with match and makes code easier to read
// we use the :: (double semi colon) to assiant the varient to the enumeration

enum Direction {
    Left,
    Right,
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }
}
