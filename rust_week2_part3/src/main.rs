// use std::io;

// This example is a useful application of `while` because it allows to continue
// asking for user input until the user types a specific word (in this case,
// "stop").
fn main() {
    // let mut input = String::new();
    // while input.trim() != "stop" {
    //     input.clear();
    //     println!("Please enter a word (type 'stop' to exit):");
    //     io::stdin().read_line(&mut input).expect("Failed to read input");
    //     println!("You entered: {}", input);
    // }
    // println!("Goodbye!");

    // for i in (1..=10).rev() {
    //     println!("i = {}", i);
    // }

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n)
    }

    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            break;
        }
    }
}