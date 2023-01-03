fn main(){
    // Declaracion de tupla de tamano 3
    let tuple_e = ('E', 5i32,true);

    // Uso de la indexaci[on de la tupla para mostrar los elementos de la tupla
    println!("Is '{}' the {}th letter of the alphabet? {}",tuple_e.0,tuple_e.1,tuple_e.2);

    //Instanciacion de las estructuras

    //Intancia de una estructura clasica
    let user_1 = Student {name: String::from("Constance Sharma"), remote: true, level:2};
    let user_2 = Student {name: String::from("Dyson Tan"), level:5,remote:false};

    // Instanciacion de una estructura de tuplas
    let mark_1 = Grades('A','A','B','A',3.75);
    let mark_2 = Grades('B','A','A','C',3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name,user_1.level,user_1.remote,mark_1.0,mark_1.1,mark_1.2,mark_1.3,mark_1.4);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
    user_2.name,user_2.level,user_2.remote,mark_2.0,mark_2.1,mark_2.2,mark_2.3,mark_2.4);
}

// Definicion de estructuras

// Estructura clasica
struct Student {name: String, level:u8, remote:bool} //No termina en punto y coma

// Estructura de tupla
struct Grades(char,char,char,char,f32); // Si termina en punto y coma