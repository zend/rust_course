use std::ops::{Deref, DerefMut};

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

  let a = Box::new(3);
  println!("a = {}", a);
  println!("a + 1 = {}", *a + 1);

  let arr = [0; 1000];
  let arr1 = arr;
  println!("arr1 is a new copy of arr, {:?} != {:?}", arr.as_ptr(), arr1.as_ptr());

  let ab = Box::new([0; 1000]);
  let ab2 = ab; // value moved here
  // println!("ab = {:?}", ab); // value borrowed here after move
  println!("ab2 = {:?}", ab2);

  let arr = vec![Box::new(1), Box::new(2)];
  let (first, second) = (&arr[0], &arr[1]);
  println!("{first}, {second}");
  println!("first + second = {}", **first + **second);


  let x = 5;
  let y = &x;
  println!("x={}, y={}", x, y);
  assert_eq!(5, x);

  struct MyBox<T>(T);
  impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
  }

  let b = MyBox::new(3);
  // println!("b = {}", *b); // cannot be dereferenced

  impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  println!("b = {}", *b);
  fn display(s: &str) {
    println!("s = {}", s);
  }

  // 所有权转移到函数内部，离开函数时自动Drop
  fn fake_drop(_b: MyBox<i32>) {}
  fake_drop(b);

  // 隐式解引用转换
  let s = String::from("value");
  display(&s);

  // 隐式解套娃引用
  let ds = MyBox::new(String::from("value"));
  display(&ds);
  let s1: &str = &ds;
  let s2: String = ds.to_string();
  println!("{s1} {s2}");

  impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
    }
  }

  fn append(s: &mut String) -> &mut String {
    s.push_str(" world");
    s
  }
  let mut s = MyBox::new(String::from("hello"));
  let s = append(&mut s);
  println!("s = {}", s);

  impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
      println!("Dropping MyBox");
    }
  }
}
