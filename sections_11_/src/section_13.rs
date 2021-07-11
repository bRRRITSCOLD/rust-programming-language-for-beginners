extern crate num;

use num::Num;

fn section_thirteen_generics_one() {
  fn largest<T:PartialOrd+Copy>(list: &[T]) -> T {
    let mut lrgst = list[0];
    for &n in list {
      if n > lrgst {
        lrgst = n; 
      }
    }
    lrgst
  }
  println!("{}", largest(&vec![1,2,3,4,5,6,7,8]));
}

#[derive(Debug)]
struct Point<T,E> {
  x: T,
  y: E,
}

fn section_thirteen_generics_two() {
  let point1 = Point{x:5,y:3};
  println!("{:?}", point1);

  let point2 = Point{x:5.1,y:3.2};
  println!("{:?}", point2);

  let point3 = Point{x:5.1,y:3};
  println!("{:?}", point3);
}

impl <T,E> Point <T,E> {
  fn x (&self) -> &T {
    &self.x
  }
}

fn section_thirteen_generics_three() {
  let point = Point{x:5.1,y:3};
  println!("{:?}", point.x());
}

impl <E> Point <f32, E> {
  fn number (&self) -> &f32 {
    &self.x
  }
}

impl <E> Point <i32, E> {
  fn number (&self) -> &i32 {
    &self.x
  }
}

fn section_thirteen_generics_four() {
  let point1 = Point{x:5.1,y:3};
  println!("{:?}", point1.number());

  let point2 = Point{x:5,y:3};
  println!("{:?}", point2.number());
}

trait Summary {
  fn summarize(&self) -> String {
    String::from("Summarized...")
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

struct Post {
  username: String,
  content: String,
}

impl Summary for Post {}

fn section_thirteen_traits_one() {
  let news_article_1 = NewsArticle {
    headline: String::from("Headline"),
    location: String::from("Location"),
    author: String::from("Author"),
    content: String::from("Content"),
  };
  println!("{}", news_article_1.summarize());

  let tweet_1 = Tweet {
    username: String::from("Username"),
    content: String::from("Content"),
    reply: false,
    retweet: false,
  };
  println!("{}", tweet_1.summarize());

  let post_1 = Post {
    username: String::from("Username"),
    content: String::from("Content"),
  };
  println!("{}", post_1.summarize());
}

fn section_thirteen_lifetimes_one() {
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  let s1 = "Hello";
  let s2 = "Bye";
  let result = longest(s1, s2);
  println!("{}", result);
}

fn section_thirteen_lifetimes_two() {
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  let s1 = "Hello";
  let s2 = "Bye";
  let result = longest(s1, s2);
  println!("{}", result);
}

struct S<'a> {
  name: &'a String,
}

impl <'a> S <'a>{
  fn fun(&self) -> &String {
    self.name
  }
}

fn section_thirteen_lifetimes_three() {
  let s1 = S {
    name: &String::from("Name"),
  };

  println!("{}", s1.fun());
}

#[derive(Debug)]
struct S2 <T:Num> {
  a: T,
  b: T,
}

impl <T:Num+Copy> S2 <T> {
  fn add(&self) -> T {
    self.a + self.b
  }
}

fn section_thirteen_lifetimes_four() {
  fn add<T:Num> (a: T, b: T) -> T {
    a + b
  }
  println!("{}", add(1, 1));

  let s2 = S2{
    a: 2,
    b: 3,
  };
  println!("{:?}", s2.add());
}

pub fn run() {
  println!("section 13 starting");

  section_thirteen_generics_one();

  section_thirteen_generics_two();
 
  section_thirteen_generics_three();

  section_thirteen_generics_four();

  section_thirteen_traits_one();

  section_thirteen_lifetimes_one();

  section_thirteen_lifetimes_two();

  section_thirteen_lifetimes_three();

  section_thirteen_lifetimes_four();

  println!("section 13 finishing");
}