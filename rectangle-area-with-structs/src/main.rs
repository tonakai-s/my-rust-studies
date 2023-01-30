#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Implement the method area on the struct.
//&self references to the instance calling the method.
//And is a short for self: &Self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    //The specifier {:?} means we want use a formater called Debug, Debug it's a trait.
    //The specifier {:#?} print the struct formated, not all info on the same line.
    println!("The rectangle is {:#?}", rectangle1);
    println!("The rectangle area is {} square pixels.", rectangle1.area());
}
