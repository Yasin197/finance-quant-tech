// The main difference between if else and match is that it must be exaustive - all options must be accounted for
// within the {} we must list out all the possible values the variable must be (my_name in the example below)
// the => means to perform an action and is known as a "fat arrow"
// so each condtion must be listed and ends in a comma (,) as this is an expression and not with ; which are for statements
// we use the _ as the last expression to encompass everything else

fn main() {
    let my_name = "Bob";
    match my_name {
        "Jayson" => print!("that is my name"),
        "Bob" => print!("not my name"),
        "Alice" => print!("hello alice"),
        _ => print!("nice to meet you!"),
    }
}