fn main() {
    let s1 = String::from("Hello");

    print_bytes(&s1);
}

fn print_bytes(s: &String){
    let _bytes = s.as_bytes();
}