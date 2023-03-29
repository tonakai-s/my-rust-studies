fn main() {
    let x = 5;
    let y = &x;

    println!("X: {}, Y: {}", x, y);
    assert_eq!(5, x);
    //Comparing a number and a reference to a number is't allowed, because
    //they are different types. So, we use the deference operator, now y
    //is pointing to the value.
    assert_eq!(5, *y);

    let a = 5;
    let b = Box::new(a);


    println!("A: {}, B: {}", a, b);
    assert_eq!(5, a);
    //The bigger difference here is that the B points to a copied value of A
    //not a true reference.
    //But we need to use the dereference operator to follow the value of the box.
    assert_eq!(5, *b);
}
