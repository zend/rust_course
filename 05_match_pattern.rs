#[derive(Debug)]
enum Direction {
  East,
  South,
  West,
  North,
}

struct Point {
  x: i32,
  y: i32,
  z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
  {
    let e = Direction::East;
    let s = Direction::South;
    let w = Direction::West;
    let n = Direction::North;
    match e {
        Direction::East => println!("East"),
        Direction::North | Direction::South => println!("SN"),
        _ => println!("West"),
    }
    let d = match e {
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::North => 4,
    };
  
    println!("d={}", d);
    println!("{:?}, {:?}, {:?}, {:?}", e, s, w, n);
  
    let v = vec![e, s, w, n];
    println!("filter {:?}", v.iter().filter(|x| matches!(x, Direction::East)));
    // Test alphabet
    assert!(matches!('f', 'a'..='z' | 'A'..='Z'))
  }

  {
    fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    if let Ok(n) = "15".parse::<i32>() {
      println!("numberic {}", n);
    }

    // let-else
    fn get_count_item(s: &str) -> (u32, &str) {
      let mut it = s.split(' ');
      let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Invalid string");
      };
      // 因为上面是 let 语句 count_str, item 在后续可用
      let Ok(count) = u32::from_str_radix(count_str, 10) else {
        panic!("count is not numberic");
      };
      (count, item)
    }
    assert_eq!(get_count_item("123 a"), (123, "a"));

    // 神奇的 let
    let p = Point {x: 1, y: 2, z: 3};
    // 解构
    let Point { x: a, y: b, z: c} = p;
    println!("a={}, b={}, c={}", a, b, c);
    // 也可以同名解构
    let Point { x, y, z } = p;
    println!("point ({}, {}, {})", x, y, z);
    // 部分解构
    let Point { x, y, z: _ } = p;
    println!("x={}, y={}", x, y);
    // 一次忽略多个
    let Point { x, .. } = p;
    println!("x={}", x);
    // 顺序无关
    let Point { z, .. } = p;
    println!("z={}", z);

    // 枚举值的解构
    let msg = Message::ChangeColor(255, 255, 0);
    let _quit = Message::Quit;
    let _move = Message::Move { x: 800, y: 600 };
    let _write = Message::Write(String::from("Hello World!"));
    match msg {
        Message::Quit => {
          println!("Quit the chat.");
        }
        Message::Move { x, y } => {
          println!("x={}, y={}", x, y);
        }
        Message::Write(text) => {
          println!("text message to write: {}", text);
        }
        Message::ChangeColor(r, b, g) => {
          println!("new color: rgb({}, {}, {})", r, b, g);
        }
    }

    let num = Some(5);
    match num {
        Some(x) if x < 5 => println!("num = {}, x < 5", x),
        Some(x) => println!("num = {}", x),
        _ => println!("Default case"),
    };
  }

  {
    impl Point {
      fn distance(&self, p: Point) -> f64 {
        let sum = (self.x - p.x).pow(2) + (self.y - p.y).pow(2) + (self.y - p.y).pow(2);
        (sum as f64).sqrt()
      }
    }
    let p1 = Point {x: 1, y: 1, z: 1};
    let p2 = Point {x: 100, y: 100, z: 100};
    println!("distance between p1 and p2: {}", p1.distance(p2));
  }
}
