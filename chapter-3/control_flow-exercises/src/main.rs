fn main() {
    let celsius = 30.0;
    let fahrenheit = 90.0;
    println!("{}º C = {}º F", celsius, celsius_to_fahrenheit(celsius));
    println!("{}º F = {}º C", fahrenheit, fahrenheit_to_celsius(fahrenheit));
    println!("Decimo elemento de Fibonacci: {}", fibonacci_recursivo(10));
    println!("Decimo elemento de Fibonacci: {}", fibonacci_iterativo(10));
    twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(x: f64) -> f64 {
    x * 9.0/5.0 + 32.0
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0/9.0
}

fn fibonacci_recursivo(x: usize) -> usize {
    if x < 2 {
        x
    } else {
        fibonacci_recursivo(x-1) + fibonacci_recursivo(x-2)
    }
}

fn fibonacci_iterativo(x: usize) -> usize {
    if x < 2 {
        x
    } else {
        let mut i = 0;
        let mut j = 1;
        let mut t;
        for _ in 1..x {
            t = i+j;
            i = j;
            j = t;
        }
        j
    }
}

fn twelve_days_of_christmas() {
    let what_gave_me = [
        " partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    println!("Twelve Days of Christmas");
    println!();
    for i in 1..13 {
        print!("On the {}", i);
        if i == 1 {
            print!("st");
        } else if i == 2 {
            print!("nd");
        } else if i == 3 {
            print!("rd");
        } else {
            print!("th");
        }
        println!(" day of Christmas");
        println!("My true love gave to me");
        for j in (0..i).rev() {
            if j == 0 {
                if i == 1 {
                    print!("A");
                } else {
                    print!("And a");
                }
            }
            println!("{}", what_gave_me[j]);
        }
        println!();
    }
}
