//Brings HashMap module into the scope.
use std::collections::HashMap;

fn main() {
    //Instantiate a new HashMap.
    let mut _scores = HashMap::new();

    _scores.insert(String::from("Blue"), 10);
    _scores.insert(String::from("Yellow"), 50);
    //This insert overwrite the value on the existent key.
    _scores.insert(String::from("Yellow"), 100);

    //This line insert Blue with value 50 ONLY IF Blue key not exists.
    _scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    //get return an Option<&T>
    //.copied transform or copy the retun of the get into an Option<T> instead <&T>
    //unwrap_or, return the value on the Some(T) or return 0 (In this case)
    //if the value is None
    let _score = _scores.get(&team_name).copied().unwrap_or(0);

    for(key, value) in &_scores{
        println!("{key}: {value}");
    }

    //=>THIS THROW AN ERROR
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(&field_name, &field_value);

    // let my_color = field_value;

    // println!("{:?}", map);
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //It return a mutable reference to the value on the hashmap
        let count = map.entry(word).or_insert(0);
        //It increments the mutable reference
        *count += 1;
    }

    println!("{:?}", map);
}