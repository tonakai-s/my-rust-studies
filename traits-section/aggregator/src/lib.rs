use std::cmp::PartialOrd;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
    pub username: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!(
            "@{}",
            self.username
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!(
            "@{}",
            self.username
        )
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking new! {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

//In this case, method 'cmp_display_largest' is only implemented
//on types that implement traits Display AND PartialOrd
impl<T: Display + PartialOrd> Partial<T> {
    fn cmp_display_largest(&self) {
        if self.x >= self.y {
            return println!("The biggest number is x = {}", self.x);
        }
        println!("The biggest number is y = {}", self.y);
    }
}

//This code implements trait 'ToString' on types that implement trait 'Display'
impl<T: Display> ToString for T{
    //snip
}