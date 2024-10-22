#[derive(Debug)]
enum Direction {
  East,
  South,
  West,
  North,
}

fn main() {
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

  
}
