use crate::garden::vegetables;

pub mod garden;

fn main(){
    let plant2 = crate::garden::vegetables::Aspargus{};
    let plant = vegetables::Aspargus{};
    let _plant3 = vegetables::Onion{};

    println!("I'm growing an {:?}", plant);
    println!("I'm growing an {:?}", plant2);
}