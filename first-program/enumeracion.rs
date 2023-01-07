/***
// Se define una enumeracion para clasificar un evento web de forma alternativa
enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enum variant can be like a classic struct with named fields and their data types
    WEClick { x: i64, y: i64 }
}***/

// Se define una enumeracion para clasificar un evento web de forma alternativa

// Se define una estructura de Tupla
#[derive(Debug)]
struct KeyPress(String, char);

// Se define una estructura clasica
#[derive(Debug)]
struct MouseClick {x:i64,y:i64}

// Se redefinen las variantes de la enumeracion para usar la data de las nuevas estructuras
#[derive(Debug)]
enum WebEventAlt {WELoad(bool),WEClick(MouseClick),WEKeys(KeyPress)}

fn main(){

    let we_load = WebEventAlt::WELoad(true);
    
    // Intanciando la estructura MouseClick con los valores de las coordenadas
    let click = MouseClick{x:100,y:250};
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Configurando la variante WEClick para usar la data en la esctructura click
    let we_click = WebEventAlt::WEClick(click);

    // Instaciar una tupla KeyPress con los valores correspondientes
    let keys = KeyPress(String::from("Ctrl+"),'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Configuracion de la variante para usar la data en la tupla de Keys
    let we_key = WebEventAlt::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
    
}