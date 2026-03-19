// Shadowing

fn main() {
    let x = 5;
    println!("Orig: {}", x);
    let x = 6;

    println!("Shadowing {} - {}", x, x);

    const Y: i32 = 5;

    // const Y: i32 = 10; // Illegal to shadow a constant
}
