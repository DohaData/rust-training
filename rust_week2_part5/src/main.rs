use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    let mut name = String::new();
    let mut numbers: Vec<i32> = Vec::new(); // Use Vec<i32> instead of [i32; 0]
    loop {
        println!("Enter a number or 'stop' to stop:");
        name.clear();
        io::stdin().read_line(&mut name).expect("failed to read input");
        if name.trim() == "stop" {
            break;
        }
        numbers.push(name.trim().parse().expect("not a number"));
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}
