use add_one;  
use add_two;  
use rand;

fn main() {
    let num = 10;
    println!("{num} plus one is {}", add_one::add_one(num));
    
    let num2 = 12;
    println!("{num2} plus two is {}", add_two::add_two(num2));
}
