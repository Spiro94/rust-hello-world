// Ownership and Borrowing

// Ownership rules:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    // 1. Each value in Rust has a variable that’s called its owner.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);

    println!("Length of {} is {}", s1, len);

    // 2. There can only be one owner at a time.
    let s2: String = s1;

    // println!("{}", s1);
    println!("{}", s2);

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("Readers {} - {}", r1, r2);

    let r3 = &mut s;
    // s.push_str("Another addition")
    r3.push_str(" Addition!");

    // println!("Test {}", s);
    s.push_str("Another addition");

    // println!("Mutator {}", r3);

    let r4 = &mut s;
    r4.push_str("Another addition");
    println!("Mutator {}", r4);

    // let x = 5;
    // let y = &mut x; //Not allowed, original variable has to be mutable as well
}

// 3. When the owner goes out of scope, the value will be dropped.
// fn printLost(s: &String) {
//     println!("{}", &s1);
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}
