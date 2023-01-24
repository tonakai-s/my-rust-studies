fn main() {
    let mut number = 3;

    while number != 0 {
        println!("Number at this moment is {number}");

        number -= 1;
    }

    println!("Jumped out of the loop!");

    //Iterating a collection.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("Entering the iterator!");
    while index < 5 {
        println!("The current value is {}", a[index]);

        index += 1;
    }

    //Another iteration, but using the for in statement.
    println!("Entering the for in iterator!");

    //This methos is better, because prevent the out of bounds exception, and remove the necessity of bool validation on each iteration, like while statement.
    for element in a {
        println!("The current element on iteration is {element}");
    }

    //A countdown using for statement.
    //(1..4) is part of the Range provided by standar library. It generate the number sequence starting at the first number, and ending BEFORE the the another number.
    //.rev() method just reverse the collection.
    for number in (1..4).rev() {
        println!("Countdown: {number}");
    }
    println!("Countdown finished.");
}
