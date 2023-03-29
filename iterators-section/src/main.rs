//Notes: Iterators are high-level loops writed without low-level cost.
fn main() {
    let v1 = vec![1, 2, 3];

    //It return a owned value of v1.
    //let v1_iter_owned = v1.into_iter();

    //It return muttable references to v1
    //let v1_iter_mut = v1.iter_mut();

    let mut v1_iter = v1.iter();
    
    println!("Got from next: {:?}", v1_iter.next());

    for value in v1_iter {
        println!("Got: {}", value);
    }

    //Iterators adaptors are methods defined on the Iterator trait
    //that don't consume the iterator, but produce a diferent changing the original.
    let v2: Vec<i32> = vec![1, 2, 3];
    let v2_mapped: Vec<_> = v2.iter().map(|num| num + 1).collect();
    println!("My mapped iterator: {:?}", v2_mapped);

    //.filter iterator adaptor need to be called with .into_iter() method.
}
