#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Implement the method area on the struct.
//&self references to the instance calling the method.
//And is a short for self: &Self
//Structs can have more then one impl associated.
impl Rectangle {
    //It's called associated function.
    //Associated functions can be associated to the constructors on the POO, since Rust don't have the NEW keyword.
    //This functions don't receive &self has a parameter.
    //And the Self on return even will be the type os the name after impl, in this case will be Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        println!("This is my self: {:?}", &self);
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    //Associated functions are called using ::, like namespaces.
    let my_square = Rectangle::square(5);

    println!("My square is {:#?}", my_square);

    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 30,
    };

    let rectangle3 = Rectangle {
        width: 31,
        height: 51,
    };

    //The specifier {:?} means we want use a formater called Debug, Debug it's a trait.
    //The specifier {:#?} print the struct formated, not all info on the same line.
    println!("The rectangle1 is {:#?}", rectangle1);
    println!(
        "The rectangle1 area is {} square pixels.",
        rectangle1.area()
    );

    println!(
        "rectangle 2 fit inside rectangle1? {}",
        rectangle1.can_hold(&rectangle2)
    );

    println!(
        "rectangle 3 fit inside rectangle1? {}",
        rectangle1.can_hold(&rectangle3)
    );
}
