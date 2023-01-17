fn main(){

    // Declaracion de un vector inicializado con 3 valores
    let three_nums = vec![15,3,46];
    println!("Initial vector: {:?}",three_nums);

    // Declaracion de un vector inicializado con valores cero y tamano 5
    let zeroes = vec![0;5];
    println!("Zeroes: {:?}", zeroes);

    // Crear un vector vacio, se declara un vector mutable para que pueda crecer o decrecer
    let mut fruit = Vec::new();
    println!("Fruits Before {:?}",fruit);

    // Push() agrega valores al final del vector
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits added {:?}",fruit);

    // Pop() elimina el valor del final del vector

}