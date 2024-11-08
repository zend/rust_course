fn main() {
  let y = 1;
  let action = |x| x + y;

  println!("{}", action(1));
  // y += 1; // `y` is assigned to here but it was already borrowed
  println!("{}", action(1));

  let add = |x, y| x + y;
  println!("{}", add(1, 2));
  // println!("{}", add("1", "2"));  // expected integer, found `&str`

  fn add1(x: i32) -> i32 { x + 1 }
  let add2 = |x: i32| { x + 1 };
  let add3 = |x| { x + 1 };
  let add4 = |x| x + 1;

  add1(1);
  add2(1);
  add3(1);
  add4(1);

  struct Cacher<T, U>
  where T: Fn(U) -> U, U: Copy
  {
    query: T,
    value: Option<U>,
  }

  impl<T, U> Cacher<T, U>
  where T: Fn(U) -> U, U: Copy {
    fn new(query: T) -> Cacher<T, U> {
      Cacher {
        query,
        value: None,
      }
    }

    fn value(&mut self, arg: U) -> U {
      match self.value {
        Some(v) => v,
        None => {
          let v = (self.query)(arg);
          self.value = Some(v);
          v
        }
      }
    }
  }

  let mut cacher = Cacher::new(|x| x);
  let v1 = cacher.value("1u32");
  let v2 = cacher.value("2u32");
  println!("v1={}, v2={}", v1, v2);
}