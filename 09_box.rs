trait Draw {
  fn draw(&self) -> String;
}

impl Draw for u32 {
  fn draw(&self) -> String {
    format!("u32: {}", *self)
  }
}

impl Draw for f32 {
  fn draw(&self) -> String {
    format!("f32: {}", *self)
  }
}

fn do_draw(x: &dyn Draw) {
  println!("do_draw: {}", x.draw());
}

fn box_draw(x: Box<&dyn Draw>) {
  println!("box_draw: {}", x.draw());
}

fn main() {
  let a: u32 = 12;
  let b: f32 = 34.5;

  println!("a {}, b {}", a.draw(), b.draw());
  do_draw(&a);
  box_draw(Box::new(&b));

  let container: Vec<&dyn Draw> = vec![&a, &b];
  for item in container.iter() {
    println!("item in box: {}", item.draw());
  }

  let boxes: Vec<Box<dyn Draw>> = vec![Box::new(a), Box::new(b)];
  for item in boxes.iter() {
    println!("item in box2: {}", item.draw());
  }
}
