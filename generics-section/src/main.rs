use std::cmp::PartialOrd;

fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 45, 5452, 523, 434];
    
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'u', 'z', 'p'];
    
    let result = get_largest(&char_list);
    println!("The largest number is {}", result);
}