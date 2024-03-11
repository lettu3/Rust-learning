fn main() {
    let x = 5;
    //this is not the same x, is a new x
    //by using the keyword _let_ we are creating a new variable
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
