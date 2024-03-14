use std::io;


fn main() {
    loop{
        println!("Please enter the number you want to convert: ");
        let mut user_input = String::new();

        //get the user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        //parse the user input
        let user_input : i32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter the measure of that number (F/C): ");
        let user_measure = input_string();


        // now, here, we calculate the conversion
        let result : i32;
        let result_measure : String;

        if user_measure.eq("F"){
            result = fahrenheit_to_celsius(user_input);
            result_measure = String::from("C")
        }
        else if user_measure.eq("C") {
            result = celsius_to_fahrenheit(user_input);
            result_measure = String::from("F")
        }
        else{
            println!("Please input a valid measure");
            continue
        }
        //then, print the result
        println!("Your input: {user_input}°{user_measure} equals: {result}°{result_measure}");
    }
}


fn fahrenheit_to_celsius(value: i32) -> i32 {
    //bounded to integer values
    let result : i32 = (value - 32) / 2; 
    return result
}

fn celsius_to_fahrenheit(value: i32) -> i32 {
    //bounded to integer values
    let result : i32 = (value * 2) + 32;
    return result
}

fn input_string() -> String {
    let mut your_string = String::new();

    io::stdin()
        .read_line(&mut your_string)
        .expect("Failed to read line");

    your_string
        .trim()
        .to_owned()
}