fn main(){
    if 1 == 2 {
        println!("True, the numbers are equal");
    }else{
        println!("False, the numbers are not equal");
    }

    let formal = true;
    let greeting = if formal { //if usado aqui como una expresion
        "Good day to you."     // return  un String
    }else{
        "Hey!"                // return un String
    };

    println!("{}",greeting);

    let num = 500; 
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    }else if num == 0 {
        out_of_range = true;
    }else if num > 512 {
        out_of_range = true;
    }else {
        out_of_range = false;
    }

    println!("{}",out_of_range);



}