fn main(){
    // Declaracion de una variable
    let mut a_number;

    // Declaracion e inicializacion de una variable
    let a_word = "Ten";

    // Inicializacion de una variable ya inicializada
    a_number = 10;

    println!("The number is {}.",a_number);
    println!("The number is {}.",a_word);

    a_number = 15;
    println!("The number mutable is {}.",a_number);

    // Variables con propiedad reemplazada

    // Declaracion de la primer variable inicializada con el nombre de shadow_num
    let shadow_num = 5;

    // Declaracion de la segunda variable inicializada reemplazando la primer variable existente shadow_num
    let shadow_num = shadow_num + 5;

    // Declaracion de la segunda variable inicializada reemplazando la segunda variable existente shadow_num
    let shadow_num = shadow_num*2;

    println!("The number with replace property is {}",shadow_num);
}