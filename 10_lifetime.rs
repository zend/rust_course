use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

struct Singer<'a> {
  name: &'a str,
}

impl<'a> Singer<'a> {
  fn sing(&self, song: &'a str) -> &str {
    println!("{} is singing {}", self.name, song);
    song
  }
}

fn concat(a: String, b: String) -> String {
  let mut s = String::new();
  s = s + &a + &b;
  return s;
}

fn main() {
  let s1 = String::from("hello");
  let s2 = "xyz";

  let result = longest(s1.as_str(), s2);
  println!("The longest string is {}", result);

  let singer = Singer { name: "Tylor" };
  let text = singer.sing("Cruel Summer");
  println!("{text}");

  let a = "hello".to_string();
  let b = "world".to_string();
  println!("concat s = {}", concat(a, b));

  fn print_author(author: &str) {
    println!("{author}");
  }
  fn print<T: Display + 'static>(msg: &T) {
    println!("msg = {}", msg);
  }
  let author = "Tylor Swift";
  let num = 15u64;
  print_author(author);
  print(&author);
  print(&num);
}