use crate::garden::vegetables::Aspargus;

pub mod garden;

fn main(){
    let plant = Aspargus{};

    println!("I'm growing an {:?}", plant);
}