fn main() {
    //Like tuples, you can access a value by their name dot index.
    //And using this, each Struct have is your own type, so a fn who receive a Color, cannot receive a Point.
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

//Here we define the name of the Struct, with their tuple values.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//It's a unit-like struct.
//A struct without any value, most used for traits.
struct AlwaysEqual;
