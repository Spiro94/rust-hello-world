fn main() {
    // If Else
    let mut x = 10;
    if x > 5 {
        println!("X is less than 5");
    } else {
        println!("X is greater than 5");
    }

    let y = 6;

    if y % 2 == 0 {
        println!("Numer is divisible by 2")
    } else if y % 3 == 0 {
        println!("Numer is divisible by 3")
    }

    let condition = true;
    x = if condition { 7 } else { 8 };

    println!("New x value {x}");

    let reward_points: Option<i32> = None;

    // If let
    if let Some(points) = reward_points {
        println!("You earned {} points!", points);
    } else {
        println!("No points earned this time.");
    }

    // Loop

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Counter {counter}");

        if counter == 10 {
            break counter;
        }
    };

    println!("Result {:?}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        println!("Element {element}");
    }
}
