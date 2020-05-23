fn main() {
    // Functions
    println!("Hello, world!");
    simple_function();
    parameter_function(5);
    parameters_function(5, 6);

    // Function bodies
    // let x = (let y = 6); - > Ilegal, no se puede meter un let en otro
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn simple_function() {
    println!("Another function.");
}

fn parameter_function(x: isize) {
    println!("The value of x is: {}", x);
}

fn parameters_function(x: isize, y: isize) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> isize {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // Poner x + 1; con punto y coma devuelve error!
}
