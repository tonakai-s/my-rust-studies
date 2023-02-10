//Given a list of integers, use a vector and return the median
//(when sorted, the value in the middle position)
//and mode (the value that occurs most often; a hash map will be helpful here)
//of the list.
use std::fs;
use std::collections::HashMap;

fn main() {
    let integers_on_file: Vec<String> = fs::read_to_string("integers-input.txt")
                        .unwrap()
                        .lines()
                        .map(str::to_string)
                        .collect();

    let mut integers_file_map = HashMap::new();
    let mut integers_vector: Vec<i32> = Vec::new();

    for int in &integers_on_file {
        let parsed_int = int.parse::<i32>().unwrap();
        integers_vector.push(parsed_int);

        let count = integers_file_map.entry(int).or_insert(0);
        *count += 1;
    }

    let mut first_most_used_number: String = String::from("");
    let mut qtd_first_most_used_number: i8 = 0;

    for(key, value) in integers_file_map {
        if value > qtd_first_most_used_number {
            qtd_first_most_used_number = value;
            first_most_used_number = key.to_string();
        }
    }

    integers_vector.sort();
    let middle_vector_index = (integers_vector.len() - 1) / 2;

    println!("The who most appear on the list is: {}", first_most_used_number);
    println!("The number of the middle is {}", integers_vector.get(middle_vector_index).unwrap());
}
