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

    let number: u32 = 14;
    println!("The number u32 is {}",number);

    let number_64 = 4.0;  // El compilador infiere el valor y usa por defecto f64
    let number_32: f32 = 5.0; // El tipo f32 se especifica con la anotacion

    println!("Number 64 {} and Number 32 {}", number_64,number_32);

    // Suma, resta y multiplicacion
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",1u32+2,8i32-5,15*3);
    println!("9 / 2 = {} but 9.0 / 2.0 = {}",9u32/2,9.0/2.0);

    // Valores booleanos
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    // Texto y caracteres
    let uppercase_s: char = 'S';
    let lowercase_f: char = 'f';
    let smiley_face = '😃';

    println!("My {}ace when I {}ee Karen is {}",lowercase_f,uppercase_s,smiley_face);
}