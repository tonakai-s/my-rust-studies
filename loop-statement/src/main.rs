fn main() {
    //This code below produces an infinity loop.
    // loop {
    //     println!("Again!");
    // }

    let mut counter = 0;

    //On make a break, the value will be returned, so you can use it to store on a variable with the same type of the value returned.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Labeled loop.
    let mut count = 0;
    //Here, the label is assigned using a single quote on the start.
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            //When remaining reach 9, it will brek the inner loop, this loop here.
            if remaining == 9 {
                break;
            }
            //But when count reach 2, it will break the outer loop, so you need to refer his label, in this case is the 'counting_up.
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");    
}