// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// Rust is an expressions-based language, they coalesce to a single point, can be used for nesting logic

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main() {
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}
