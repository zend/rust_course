use std::fmt::{Debug,Display};

pub trait Summary {
  fn get_title(&self) -> String;
  fn summarize(&self) -> String {
    format!("《{}》", self.get_title())
  }
}

pub struct Post {
  title: String,
  content: String,
}

#[derive(Debug)]
pub struct Email {
  subject: String,
  body: String,
}

#[derive(Debug)]
pub struct Tweet {
  content: String,
}

impl Summary for Post {
  fn get_title(&self) -> String {
    format!("{}", self.title)
  }
  fn summarize(&self) -> String {
    format!("{} {}", self.title, self.content)
  }
}

impl Summary for Email {
  fn get_title(&self) -> String {
    format!("{}", self.subject)
  }
  fn summarize(&self) -> String {
    format!("{} {}", self.subject, self.body)
  }
}

impl Summary for Tweet {
  fn get_title(&self) -> String {
    format!("{}", self.content)
  }
}

impl Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "subject => {}, body => {}", self.subject, self.body)
  }
}

fn read(item: &impl Summary) {
  println!("I'v read {}", item.summarize());
}

fn share<T: Summary + Debug>(item: &T) {
  println!("Sharing item {:?}", item);
}

// 用 where 的写法
fn share2<T>(item: &T) 
  where T: Summary + Debug 
{
  println!("Sharing item {:?}", item);
}

fn get_random_content() -> impl Summary {
  Tweet {
    content: String::from("Elon Musk"),
  }
}


fn main() {
  let post = Post {
    title: String::from("Hello"),
    content: String::from("World"),
  };
  let email = Email {
    subject: String::from("通知"),
    body: String::from("明天下午三点开会"),
  };
  let tweet = Tweet {
    content: String::from("推特"),
  };

  println!("post => {}", post.summarize());
  println!("email => {}", email.summarize());
  println!("Tweet => {}", tweet.summarize());
  println!("Email = {:?}", email);
  println!("Email = {}", email);

  read(&post);
  share(&tweet);
  // share(&email); // Error, `Email` doesn't implement `Debug`
  share2(&tweet);

  println!("random {}", get_random_content().summarize());
}
