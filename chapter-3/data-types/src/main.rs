use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    //this code do not check if index is in the bounds, it will try to access invalid memory
    //and then panic
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
