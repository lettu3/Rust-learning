use std::io;


fn main() {

    loop{
        println!("Please enter your n-th fibonacci number: ");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input : u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };   

        let fibonacci_user = fibonacci(user_input);
        println!("The {user_input}th fibonacci number is: {fibonacci_user}");
    }    
}

fn fibonacci(input : u32) -> u32{
    let result : u32;
    if input == 0 {
        result = 0;
    }
    else if input == 1{
        result = 1;
    }
    else {
        result = fibonacci(input - 1) + fibonacci(input - 2);
    }
    return result;
}
