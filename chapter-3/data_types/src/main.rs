fn main() {
    // DATA TYPES
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);
    // Daria error si no se define un tipo
    // let guess = "42".parse().expect("Not a number!");

    // SCALAR TYPES
    let x: isize = -42;
    let y: usize = 42;
    println!("isize: {}, usize: {}", x, y);

    println!("Decimal: {}", 98_222);
    println!("Hex: {}", 0xff);
    println!("Octal: {}", 0o77);
    println!("Binary: {}", 0b1111_0000);
    println!("Byte: {}", b'A');

    // FLOATING-POINT TYPES
    let a = 2.0; // f64
    let b: f32 = 3.0; // f32
    println!("f64: {}, f32: {}", a, b);

    // OPERACIONES
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // BOOLEANOS
    let t = true;
    println!("True: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("False: {}", f);

    // CHARACTERS
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}",c,z,heart_eyed_cat);

    // TUPLAS
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 1a forma de acceso
    println!("({}, {}, {})", tup.0,tup.1,tup.2);

    // 2a forma de acceso
    let (a1, a2, a3) = tup;
    println!("({}, {}, {})", a1,a2,a3);

    // 3a forma de acceso
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("({}, {}, {})", five_hundred,six_point_four,one);

    // ARRAYS
    let l1 = [1, 2, 3, 4, 5];
    print!("L1 = [");
    for i in &l1 {
        print!("{} ", i);
    }
    println!("]");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    print!("Months = [");
    for i in &months {
        print!("{} ", i);
    }
    println!("]");

    let l2: [i32; 5] = [1, 2, 3, 4, 5];
    print!("L2 = [");
    for i in &l2 {
        print!("{} ", i);
    }
    println!("]");

    let l3 = [3; 5];
    print!("L3 = [");
    for i in &l3 {
        print!("{} ", i);
    }
    println!("]");

    // Similar a let l3 = [3, 3, 3, 3, 3];
    let first = l1[0];
    let second = l1[1];
    println!("first = {}, second = {}", first, second)
    // let element = l1[10]; Fallaria porque el 10 se sale de la lista
    // El array solo tiene 5 elementos
}
