fn main() {
    let s1 = String::from("hello");

    // & crea una referencia que apunta a s1
    // pero no se apropia de s1.
    // Por tanto, no se pierde en el ambito
    // de la nueva funcion.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    println!("The length of '{}' is {}.", s1, s1.len());

    // Esto es ilegal.
    // let s = String::from("hello");
    // change(&s);
    // No se puede cambiar un valor referenciado,
    // ni siquiera su referencia.

    let mut s = String::from("hello");
    change(&mut s);
    println!("The length of '{}' is {}.", s, s.len());

    // Es ilegal porque no se puede mutar un valor mas de una vez
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Para cambiar una referencia, hay que utilizar mut,
// de otra forma es ilegal
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
