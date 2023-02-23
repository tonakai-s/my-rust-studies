use std::fmt::Display;

//This code won't compile, because the borrow checker
//doesn't know if the return is a reference for word1, or word2.
//So he can't define the lifecicle.
    // fn longest(word1: &str, word2: &str) -> &str {
    //     if word1.len() >= word2.len(){
    //         word1
    //     } else {
    //         word2
    //     }
    // }

//Lifetime genetic syntax
//&i32
//&'a i32
//&'a mut i32

fn longest<'a>(word1: &'a str, word2: &'a str) -> &'a str {
    //This throw an error, because the value not have relationship whit return value or parameters.
    //So the value has been created inside the function.
        // let result = String::from("A really long string.");
        // result.as_str()

    if word1.len() >= word2.len(){
        word1
    } else {
        word2
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn annouce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention, please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display{
        println!("Announcement: {}", ann);
        if x.len() >= y.len(){
            x
        } else {
            y
        }
    }

fn main() {
    // let word1 = String::from("abcd");
    // let word2 = "xyz";

    // let longest_slice = longest(word1.as_str(), word2);

    // println!("The biggest slice is {}", longest_slice);

    let my_part = String::from("Test string part");
    let my_instance = ImportantExcerpt {
        part: my_part.as_str()
    };

    //This throws an error because my_instance is used after the move of my_part.
        // let second_part = my_part;

    println!("{:#?}", my_instance);

    let announcement = String::from("My announcement");
    my_instance.annouce_and_return_part(announcement.as_str());

    let word3 = String::from("CAVALO");
    let word4 = "TESTE";

    longest_with_announcement(word3.as_str(), word4, announcement);
}
