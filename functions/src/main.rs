//This is a function, the entry point of a program on Rust.
fn main() {
    let y = {
        let x = 6;
        x + 1
    };

    println!("Y value {y}");

    simple_print();
    print_the_number(5);

    println!("Print the value of the function, and is {}", six());

    //It is a perfect assignment, because a function return a value to insert into the variable.
    let updated_one = plus_one(6);
    print!("Updated_one variable value is {updated_one}");
}

//A simple function.
fn simple_print(){
    println!("A random word, LOL");
}

//A simple function with a parameter. Is needed inform the data type os all parameters on a function.
fn print_the_number(x: i32){
    println!("The number informed is {x}");
}

//A simple function with a return expression.
//The return type is defined after the arrow (->).
//Is not needed use the return keyword, because the last expression is implicitily the return on Rust.
//If is added a semicolon at the end of the expression, it turns into a statement, so doesn't return a value.
fn six() -> u32 {
    6
}

fn plus_one(num: i32) -> i32 {
    num + 1
}