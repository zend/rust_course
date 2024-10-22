use std::fmt;

fn main() {
  {
    let s = String::from("Hello World!");
    let hello = &s[0..5];
    let world = &s[6..11];
  
    println!("[{}]", hello);
    println!("[{}]", world);
    println!("[{}]", &s[..5]);
    println!("[{}]", &s[6..]);
    println!("[{}]", &s[..]);

  }

  {
    let s = "想陪你走到天涯海角～";
    println!("len={}", s.len());
    println!("{}, {:?}", &s[0..3], s.char_indices());
  
    let s = "नमस्ते";
    println!("{} char indices = {:?}", s, s.char_indices());
    println!("{:?}", s.chars());
  }

  {
    let s = "I like rust. Learning rust is my favorite!";
    let ns = s.replace("rust", "RUST");
    dbg!(ns);
    let rs = s.replacen("rust", "RUST", 1);
    dbg!(rs);
  }

  {
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    let s3 = format!("{} {}", s1, s2);

    // 不转移所有权
    println!("{}, {}, {}", s1, s2, s3);

    // 转移所有权
    let s4 = s1 + " " + &s2 + " " + &s3;
    println!("s4={}", s4);
    // println!("{}", s1);
  }

  {
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    let s3 = format!("{} {}", s1, s2);

    // 不希望转移的话
    let s4 = String::new() + &s1 + " " + &s2 + " " + &s3;
    println!("s4={}", s4);
    println!("{}", s1);
  }

  {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r####"A string with "# in it. And even "##!"####;
    println!("{}", longer_delimiter);
  }

  {
    struct User {
      username: String,
      email: String,
    }
    let user = User {
      username: String::from("mikeliang"),
      email: String::from("combbs@gmail.com"),
    };
    println!("user={} <{}>", user.username, user.email);

    #[derive(Debug)]
    struct Animal {
      name: String,
      weight: u32,
    }

    let dog = Animal {
      name: String::from("dog"),
      weight: 12,
    };
    println!("dog={:#?}, name={}, weight={}", dog, dog.name, dog.weight);

    // 减肥成功后
    let dog2 = Animal {
      weight: 10,
      ..dog
    };
    println!("New doggie: {:?}", dog2);

    impl fmt::Display for Animal {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "name={}, weight={}", self.name, self.weight)
        }
    }

    println!("fmt Animal => {}", dog2);
  }
  
  // Tuple Structs
  {
    struct Color (u8, u8, u8); // RGB
    struct Size (u32, u32);
    struct Point (u32, u32, u32);

    let color = Color(255, 255, 255);
    let size = Size(800, 600);
    let point = Point(0, 0, 0);

    println!("color RGB({}, {}, {})", color.0, color.1, color.2);
    println!("size {}x{}", size.0, size.1);
    println!("point ({}, {}, {})", point.0, point.1, point.2);
  }

  // Enum
  {
    #[derive(Debug)]
    enum PokerSuit {
      Spades(u8),
      Hearts(u8),
      Clubs(u8),
      Diamonds(u8),
    }

    let spades = PokerSuit::Spades(12);
    let heart = PokerSuit::Hearts(9);
    let clubs = PokerSuit::Clubs(14);
    let diamonds = PokerSuit::Diamonds(8);
    if let (PokerSuit::Spades(a), 
      PokerSuit::Hearts(b), 
      PokerSuit::Clubs(c), 
      PokerSuit::Diamonds(d)) = (spades, heart, clubs, diamonds) {
      println!("{} {} {} {}", a, b, c, d);
    }
  }

  {
    let a = [1, 2, 3, 4, 5];
    let b = [8; 5];
    let s: [String; 5] = std::array::from_fn(|i| format!("str {}", i));
    println!("{:?} {:?} {} {:?}", a, b, a[4], s);

    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
  }
}
