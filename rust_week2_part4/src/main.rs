use std::io;


// Match control flow
//
//
fn main() {
    println!("please enter a greeting:");
    let mut name = String::new();
    io:: stdin().read_line(&mut name).expect("failed to read input");
    // let name = "Hello";

    // use of match expression to pattern match against varible "name"
    match name.to_lowercase().trim() {
        "good morning" => println!("Top of the morning to you too!"),
        "good evening" => println!("Have a nice evening!"),
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet!"),
        _ => println!("I can't find a greating, good bye."),
    }

}