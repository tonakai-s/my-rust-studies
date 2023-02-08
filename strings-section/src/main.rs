#[allow(dead_code)]
fn main() {
    //This create a new String instance
    let _my_instance_string = String::new();

    let some_data = "This is a string literal";
    let _transformed_string = some_data.to_string();

    //The method also works on a literal directly.
    let _my_string_direct_from_literal = "This is a string literal".to_string();

    //And is the same thing doing.
    let mut string = String::from("Hello, ");
    string.push_str("world");

    println!("{}", string);
    let string_to_concat = String::from(" And now concatenating more!");

    string.push_str(&string_to_concat);
    println!("{}", string);
    println!("{}", string_to_concat);

    let mut lol = String::from("lo");
    lol.push('l');
    println!("{}", lol);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{s1}-{s2}-{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

}
