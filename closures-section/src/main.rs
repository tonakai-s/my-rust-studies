use std::{thread, vec};

fn main(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //This closure implment the FnOnce.
    //This move the argument list into the thread, because
    //it can be still running while the main function not exist.
    //If it happens, the reference inside the thread won't be valid anymore.
    //Causing an error.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("After defining closure: {:?}", list);
    let immutable_list = vec![1, 2, 3, 4];
    println!("Printing before defining the closure");

    let only_borrows = || println!("From closure: {:?} ", immutable_list);

    println!("Before calling immutable closure: {:?}", immutable_list);
    //This closure implemente trait Fn.
    //Because receive an immutable reference.
    only_borrows();
    println!("After calling the immutable closure: {:?}", immutable_list);

    let mut muttable_list = vec![1, 2, 3, 4, 5];
    println!("Before defining muttable closure: {:?}", muttable_list);

    //This closure implement the FnMut
    let mut borrow_mutably = || muttable_list.push(7);
    
    borrow_mutably();
    println!("After calling muttable closure: {:?}", muttable_list);
    //It not is valid, because a immutable reference can't between an muttable reference
    //to the same value.
    //borrow_mutably();
}
