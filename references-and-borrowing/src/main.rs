fn main() {
    let s1 = String::from("OHMAGAAAA");
    let len = get_word_length(&s1); //& means to pass a references of the variable s1 to inside the funcion.
    println!("The size of the word {s1} is {len}");

    //To reference be muttable, variable need to have mut keyword, and algo the param passed need to have &mut.
    let mut s2 = String::from("Hello");
    //Big &mut restriction: If you have a mutable reference to a value, you can have no other references to that value. So the code bellow throw an error.
    // let ref1 = &mut s2;
    // let ref2 = &mut s2;
    append_str_to_param(&mut s2);
    println!("{}", s2);

    let mut _s3 = String::from("Testing references");

    let _r1 = &_s3; //No problem
    let _r2 = &_s3; //No problem
                    //let r3 = &mut s3; BIG PROBLEM
}

fn get_word_length(str: &String) -> usize {
    //&String means that the type os the parameter, is a reference to a variable of type String.
    //Since this is a borrow, the function not own the parameter, it's only a reference.
    //So by default the references, like variables, are immutables.
    str.len()
}

//Creating a mutable reference
//The type os the param, need to be a reference, and muttable, so we add &mut.
fn append_str_to_param(str: &mut String) {
    str.push_str(", world!");
}
