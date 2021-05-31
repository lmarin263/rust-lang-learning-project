fn main() {
    // No acostumbrarse a usarlos porque no siempre se va a conocer su valor
    // let s = "Hello";
    // De esta forma podemos almacenar un valor de texto desconocido
    let s = String::from("Hello");
    println!("{}, world!", s);

    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    
    // Ilegal porque Rust MUEVE, no copia, el valor del puntero de s1 a s2
    // let s1 = String::from("hello");
    // let s2 = s1;

    // Se clona porque es un valor dinamico
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // No hace falta clonarlos porque son valores primitivos
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Se crea s
    let s = String::from("hello");

    // La funcion toma la referencia de s
    takes_ownership(s);
    // s deja de ser valido a partir de este alcance

     // Se crea x
    let x = 5;

    // La funcion toma x...
    makes_copy(x);
    // ... y x sigue existiendo porque es un valor copiable
    // Por tanto, x sigue existiendo en este alcance

    let s1 = gives_ownership();
    println!("{}", s1);
    // El valor de s1 pasa del alcance de gives_ownership a este

     // Se crea s2
    let s2 = String::from("hello");

    // La funcion takes_and_gives_back toma la referencia de s2...
    let s3 = takes_and_gives_back(s2);
    // ... y la devuelve al valor s3
    println!("{}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}
// Aqui termina el alcance tanto de x como s, pero como s ya se paso a
// otro alcance, por tanto, no importa... Tambien se pierden s1 y s3.
// s2 se perdio del alcance y por lo tanto, no pasa nada.

// gives_ownership va a pasar el valor que genera
// al alcance de a aquel que lo llame.
fn gives_ownership() -> String {
    // Se crea some_string
    let some_string = String::from("hello");
    // Se devuelve some_string y se pasa al siguiente alcance.
    some_string
}

// takes_and_gives_back va a recibir y devolver un String
fn takes_and_gives_back(a_string: String) -> String {
    // Un String entra en este alcance...
    a_string
    // ... e igual que entro, se devuelve a aquel que la invoco.
}

// some_string entra en el alcance de la funcion
fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
}
// Aqui, se pierde el alcance de some_string.
// Se libera de memoria.

  // some_integer esta en este alcance
fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
}
// Aqui, se pierde el alcance de some_string pero
// no pasa nada porque es un valor que se puede copiar.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() devuelve la longitud de un String

    (s, length)
}
