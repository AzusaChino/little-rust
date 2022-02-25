use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from("loading...")
    }
}

pub trait TV: Display {}

struct Point<T> {
    x: T,
    y: T,
}

pub fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("breaking new! {}", item.summarize())
}

pub fn notifying<T: Summary + Display>(_item: &T) {}

fn some_function<T, U>(_t: &T, _u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {
    0
}

// fn returns_summary(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: "".to_string(),
//             location: "".to_string(),
//             author: "".to_string(),
//             content: "".to_string(),
//         }
//     } else {
//         Tweet {
//             username: "".to_string(),
//             content: "".to_string(),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

// default impl
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only when T is Display & PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{}", self.x);
        } else {
            println!("{}", self.y)
        }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn important() {
    let novel = String::from("from ");
    let first_sentence = novel.split(' ').next().expect("Could not find space");
    let _i = ImportantExcerpt {
        part: first_sentence
    };
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}