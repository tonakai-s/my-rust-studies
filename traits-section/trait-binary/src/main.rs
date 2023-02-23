use aggregator::{Summary, Tweet, NewsArticle, notify};

struct MediumBlogPost {
    title: String,
    content: String,
    username: String,
}

impl Summary for MediumBlogPost {
    fn summarize(&self) -> String {
        format!(
            "{}",
            self.summarize_author()
        )
    }

    fn summarize_author(&self) -> String {
        format!(
            "@{}",
            self.username
        )
    }
}

fn main() {
    let tweet = Tweet{
        username: String::from("renas"),
        content: String::from("Cabelo de careca"),
        reply: false,
        retweet: false,
    };

    let news_post = NewsArticle {
        headline: String::from("My headline"),
        location: String::from("Capivari"),
        author: String::from("O próprio"),
        content: String::from("Paralelepipedo"),
        username: String::from("renas")
    };

    let new_post = MediumBlogPost {
        title: String::from("Adolino matarrato"),
        content: String::from("Catapimbas"),
        username: String::from("renas"),
    };

    println!("{}", tweet.summarize());
    println!("{}", news_post.summarize());
    println!("{}", new_post.summarize());
    println!("{:?}", notify(&news_post));
}