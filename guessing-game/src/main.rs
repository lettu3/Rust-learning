use std::io;    //we bring the input/output library from the std library

fn main() {    //main function, no parameters
    println!("Guess the number!");    //println! macro
    println!("Please input your guess next.");
    
    let mut guess = String::new();    //user input variable
    
    io::stdin()    //we call the stdin function from the io library
        .read_line(&mut guess)     //read_line method, we pass the reference (&) to the guess variable as an argument to store the result there
        .expect("Failed to read line");    //expect method, if read_line fails, crash and print the argument of expect(),     

    println!("You guessed: {guess}");    //{} allow us to print variables
}
