use std::fmt::Result;
use std::io::Result as IOResult;

fn function1() -> Result { Ok(()) }

fn function2() -> IOResult<()> { Ok(()) }

//Create a shortcut to the module.
//So we only need access threfore the hosting, instead all path.
//NOTE WITH PUB: Before this, external users need to catch function add_to_waitlist() using:
//  restaurant::font_of_house::hosting::add_to_waitlist();
//NOTE WITH PUB: Noe, it only need to call:
//  restaurant::hosting::add_to_waitlist();
pub use crate::front_of_house::hosting;

pub mod front_of_house;
pub mod using_front;

pub fn eat_in_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //This works because the use definicion created above.
    hosting::add_to_waitlist();

    let order1 = back_of_house::Apptizer::Salad;
    let order2 = back_of_house::Apptizer::Soup;
}

mod Customer {
    pub fn eat_at_restaurant() {
        //The call need to be on the same scope os use declaration.
        super::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    //When enum is public, all the variants are public.
    pub enum Apptizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        //Because the struct have a private field, we need to declare a constructor method.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        //super keyword start the path on parent module, wich in this case is the crate root.
        super::deliver_order();
    }

    fn cook_order() {}
}