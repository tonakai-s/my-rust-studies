fn main() {
    let number = 6;

    //A simple if else statement.
    if number <= 4 {
        println!("The variable number is less or equal 4");
    } else {
        println!("The variable number if greater then 4");
    }

    //Rust not have thruthy and falsy values like Javascript.
    if number != 0 {
        println!("Number was something other than zero.");
    }

    //A simple else if statement.
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    //The return of the if would be stored on the variable.
    let new_number = if number > 5 { 10 } else { 0 };
    
    //The both values (on if and else) need to have the same type, instead it will throw an error at compile. The line below show this.
    //let new_number = if number > 5 { 10 } else { "0" };

    println!("The value of number is {new_number}");


}
