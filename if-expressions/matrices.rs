fn main(){

    // Inicializacion de un array con valores separados por comas
    let days = ["Lunes","Martes","Miercoles","Jueves","Viernes","Sabado","Domingo"];

    // Declarar un array con los valores inicializados en cero y un tamano de 5
    let bytes = [0;5];

    println!("Day 1 {}",days[0]);
    println!("Byte 1 {}",bytes[0]);

    println!("Days {:?}",days);
    println!("Bytes {:?}",bytes);
}