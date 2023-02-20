use std::{
    fs,
    io::{self, Read},
};

use std::net::IpAddr;

fn main() {
    //Macro panic! panic the program and abort all.
    // panic!("All going wrong...");

    //This return an Out of Bounds error.
    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("Proble on trying to create the file: {:?}", error)
    //         },
    //         other_error => {
    //             panic!("A problem ocurred on opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    //ANOTHER APPROACH TO USE INSTEAD NESTED MATCHES.
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // println!("{:?}", greeting_file);

    //In this case, instead panic the error, we return them to caller trait him.
    //This is called, propagation errors.

    // #[derive(Debug)]
    // #[allow(dead_code)]
    // struct OurError {
    //     message: String,
    // }

    // impl From<io::Error> for OurError {
    //     fn from(value: io::Error) -> Self {
    //         OurError {
    //             message: value.to_string(),
    //         }
    //     }
    // }

    let teste = read_username_from_file();

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        fs::File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)

        //This function:
        //Open the file
        //Create a new String internally
        //Reads the content of the file
        //Put the content into that String
        //AND RETURN IT
        //WITH ERROR HANDLING USING (?)
        // fs::read_to_string("hello.txt")
    }
    println!("{:#?}", teste);

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    pub fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{:?}", type_of(home));

    println!("{:?}", last_char_of_first_line("\nTeste"));
    
}
