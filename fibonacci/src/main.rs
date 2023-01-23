fn main() {
    generate_fibo(20);
}

fn generate_fibo(stop_number: u32){
    let mut index: u32 = 3;
    let mut prev_number: u32 = 2;

    println!("1");
    println!("2");

    while index < stop_number {
        let fibo_number: u32 = index + prev_number;

        prev_number = index;
        index = fibo_number;
        
        if fibo_number <= stop_number {
            println!("{fibo_number}");
        }
    }
}