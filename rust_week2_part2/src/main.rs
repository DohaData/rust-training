fn main() {
    let mut x = 1;

    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }

    let maybe_number: Option<Option<()>> = Some(None);
    if let Some(number) = maybe_number {
        println!("the number is {:?}", number)
    } else {
        println!("there is no number ")
    }

    let mut i = 0;
    while i < 5 {
        println!("i = {}", i);
        i += 1;
    }
}
