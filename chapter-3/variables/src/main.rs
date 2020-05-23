fn main() {
    // VARIABLES Y MUTABILIDAD
    let mut x = 5;
    println!("The value of x is: {}", x);
    // x = 6; -> cannot assign twice to immutable variable
    // si se ha declarado como let x = 5
    x = 6;
    println!("The value of x is: {}", x);

    // CONSTANTES
    const MAX_POINTS: u32 = 100_000;
    println!("Max. points: {}", MAX_POINTS);

    // SHADOWING 1
    let z = 5;
    println!("The value of z is: {}", z);

    let z = z + 1;
    println!("The value of z is: {}", z);

    let z = z * 2;
    println!("The value of z is: {}", z);

    // SHADOWING 2
    // Permitido porque reusa el nombre pero cambia el tipo
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Numero de espacios: {}", spaces);

    // NO permitido porque son de dos tipos distintos...
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
