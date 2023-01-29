fn main() {
    //Without SLICE.
    let my_str = String::from("Testing a slice in Rust");
    let first_slice = first_word(&my_str); //It is bad, because not follow if the string changes.
    println!("{}", first_slice);

    //Using SLICE.
    let hello_world = String::from("Hello World");
    let native_slice = nice_first_word(&hello_world);
    //hello_world.clear(); This throws an error because an immutable reference is borrowed above.

    println!("{}", native_slice);

    let string_literal = "Hey boys!";
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    //When using type &str instead &String on fn param.
    let native_slice_literal = nice_first_word(string_literal);
    println!("{}", native_slice_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //.as_bytes methos convert the string to an array of bytes.

    //.iter() method, iterate through the bytes array.
    //.enumerate() methos wraps the current element on a tuple continaing  (index, and a reference to the current element)
    //on the for (index, &item) we are mading a destructuring of that tuple.
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    s.len()
}

fn nice_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}
