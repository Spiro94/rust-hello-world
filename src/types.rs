// Primitives types:
// int (8-128), float (32-64), bool, char

// Compound data types:
// arrays, tuples, slices, and strings

fn main() {
    // let x: i32 = 42;
    // let y: u64 = 100;
    // println!("x: {}, y: {}", x, y);

    // let pi: f32 = 3.14;

    // println!("pi: {}", pi);

    // let is_true: bool = true;

    // println!("is_true: {}", is_true);

    // let c: char = 'A';

    // println!("c: {}", c);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    // let mix = [1,2, "apple", true];

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("fruits: {:?}", fruits);
    println!("fruits[0]: {}", fruits[0]);
    println!("fruits[1]: {}", fruits[1]);
    println!("fruits[2]: {}", fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("human: {:?}", human);
    println!(
        "Name: {}, Age: {}, Is Student: {}",
        human.0, human.1, human.2
    );

    let my_mix_tuple = ("Hello", 42, 3.14, ["apple", "banana"]);
    println!("my_mix_tuple: {:?}", my_mix_tuple);

    let number_silces: &[i32] = &[1, 2, 3, 4, 5];
    println!("number_silces: {:?}", number_silces);

    // Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold says, {}", stone_cold);

    stone_cold.push_str("Yeah");

    println!("Stone Cold says, {}", stone_cold);

    // &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("Slice value: {}", slice);

    hello_world();
    human_id("Joel", 55, 172.9);

    let result: i32 = add_numbers(1, 2);
    println!("Result: {}", result);

    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
}

// Functions
// functions and variables are snake_case

fn hello_world() {
    println!("Hello, World!");
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I'm {} years-old, and my height is {} cm",
        name, age, height
    );
}

// Expressions and Statements
// Expressions: Anything that returns a value
//      5, true & false, add(3,4)
// Statement: Anything that does NOT return a value
//      Almost all statements in Rust end with ;
//      Variable declarations: let x = 5;
//      Function definitions: fn foo(){}
//      Control flow statements: if, while, etc

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // Expression, no need for return
}
