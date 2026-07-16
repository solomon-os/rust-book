use std::fmt::{Debug, Display};

trait Summary {
    fn summarise(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    print_summary(&post);

    let x = 10;
    let y = 5;
    let max = max(&x, &y);
    println!("print max: {max}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for v in list {
        if v > largest {
            largest = v;
        }
    }

    largest
}

fn print_summary<T>(summarisable: &T)
where
    T: Summary + Debug,
{
    println!("1 new post: {}", summarisable.summarise());
}

fn max<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: std::cmp::PartialOrd,
{
    if x > y { x } else { y }
}
