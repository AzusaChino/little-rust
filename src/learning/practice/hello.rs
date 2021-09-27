fn main() {
    println!("Hello Rust");
    first();
}

fn first() {
    let a_number = 10;
    let a_boolean = true;

    println!("The number is {}.", a_number);
    println!("The boolean is {}.", a_boolean);

    a_number = 13;
    println!("{}", a_number);

    let mut b_number = 10;
    b_number = 15; // OK
}

fn shadow() {
    // The first binding is created with the name "number"
    let number = 5;

    // A different binding shadows the name "number"
    let number = number + 5;

    // Again, another new binding is created
    let number = number * 2;
    println!("The number is {}.", number);
}