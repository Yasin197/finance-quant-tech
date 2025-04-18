// Tuple example

enum Access {
    Full,
}

// example of a tuple returning 1, 2, 3
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    // example of mixing different types of data using tuples
    let (employee, access) = ("Jake", Access::Full);

    // example 2
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    // example to destructure if you are working with tuples
    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);
}
