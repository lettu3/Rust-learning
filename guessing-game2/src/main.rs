use std::io;
use rand::Rng;   //Rng trait, from the rand library
use std::cmp::Ordering;    //Ordering type


fn main() {
    println!("Guess the secret number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);    //rand::thread_rng() gets a random number that is local to the current thread and is seeded by the OS, in the 1-100 range by the gen_range method

    loop{
        println!("Please enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){   //we are converting guess from a String type to a un Unsigned 32-bits integer using the trim method(it removes all the blank spaces of the string) and the parse method (it effectively parses the string)
            Ok(num) => num,     //if Result value is Ok, just return the num value from parse
            Err(_) => continue, //if Result value is Err, make another iteration of the loop
        }; 
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal=> {
                println!("You Win");
                break;  //out of the loop
            }
        }
    }

}
