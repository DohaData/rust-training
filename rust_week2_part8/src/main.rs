use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let test = String::from("Hello, World!");
    let mut write_file = File::create("test.txt").expect("Error creating file");
    match write_file.write(test.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(error) => panic!("Error writing to file: {}", error)
    }
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                },
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
                },
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}