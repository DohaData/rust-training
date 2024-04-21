
fn main() {
    let mut height = 190;
    height -= 20;

    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("result: {}", result);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("health: {}", health);

    let health = if height < 180 {true} else {false};
    println!("health: {}", health);

}
