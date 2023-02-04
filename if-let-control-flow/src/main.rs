fn main() {
    let my_some_number = Some(3u8);
    if let Some(num) = my_some_number {
        println!("Handled number {num}");
    }
}
