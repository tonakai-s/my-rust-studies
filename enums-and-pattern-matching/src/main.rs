#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn coin_to_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
impl Message {
    fn call(&self) {
        // println!("{}", self.Write);
    }
}

#[allow(unused)]
fn main() {
    //With :: we access one variant inside the enum.
    //We use this because each variant is namespaced inside the identifier.
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    //Calling the method of the enum defined by an impl
    m.call();

    //With Option enum we can define traits for when the value is null, or is present.
    //Using Some, him will be executed when the value exists.
    //And None and him is null.
    let some_number = Some(5);
    let some_char = Some('R');

    let absent_number: Option<i32> = None;

    let prefix = String::from("Hello");
    let postfix = Some(", world.");
    let right_postfix = ", world.";

    let my_coin = Coin::Nickel;
    println!("{}", coin_to_cents(my_coin));

    coin_to_cents(Coin::Quarter(UsState::Arizona));

    let five: Option<u32> = Some(5);
    let six: Option<u32> = plus_one(five);
    println!("Value returned by Plus_one function: {:?}", six);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player()
    }

    let none = plus_one(None);
    //It don't work, because String and Option<String> are different types.
    // let phrase = prefix + postfix;
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player() {}