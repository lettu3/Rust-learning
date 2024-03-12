use std::io;


fn main() {
    let mut result : i32;

    loop{
        println!("Porfavor indique que valor quiere convertir:");
        let mut input_value = String::new();
    
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");
        
        let input_value : i32 = match input_value.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Porfavor, indique si ese valor esta en fahrenheit o celsius (minusculas)");
    
        let mut input_measure = String::new();
    
        io::stdin()
            .read_line(&mut input_measure)
            .expect("Failed to read line");
        
        
    }
    
}


fn fahrenheit_to_celsius(value: i32) -> i32 {
    let result : i32 = (value - 32) * (5/9);
    return result
}

fn celsius_to_fahrenheit(value: i32) -> i32 {
    let result : i32 = (value * (9/5)) + 32;
    return result
}