#[allow(dead_code)]
fn main() {
    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(5);

    let mut vec_with_content = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vec_with_content[2];
    println!("The third element is {third}");

    //get always return an Option<&T>
    let third: Option<&i32> = vec_with_content.get(2);
    match third {
        Some(third) => println!("The third element on match is {third}"),
        None => println!("Nothing to show"),
    }

    let _does_not_exist = vec_with_content.get(100);
    // let does_not_exist = &vec_with_content[100];

    for num in &vec_with_content {
        println!("{num}");
    }

    for num in &mut vec_with_content {
        *num += 50;   
    }

    for num in &vec_with_content {
        println!("{num}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let _spreadsheet_row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.11),
        SpreadsheetCell::Text(String::from("My row"))
    ];


}
