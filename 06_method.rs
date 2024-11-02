fn main() {
  struct Rectangle {
    width: u32,
    height: u32
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  let rect = Rectangle { width: 100, height: 80 };
  println!("area = {}", rect.area());

  #[derive(Debug)]
  struct Rect<T> {
    width: T,
    height: T,
  }
  impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
      self.width * self.height
    }
  }

  let rect = Rect { width: 10.5, height: 20.7};
  println!("{:?} w={}, h={}, area={}", rect, rect.width, rect.height, rect.area());

  enum Vehicle<T> {
      Car(T),
      Wagon(T),
      Bicycle(T),
  }
  let _car = Vehicle::Car("Tesla");
  let wagon = Vehicle::Wagon((2, "horses"));
  let _bike = Vehicle::Bicycle(2); // Two wheels

  match wagon {
      Vehicle::Wagon((cnt, power)) => {
        println!("Wagon driven by {} {}", cnt, power);
      }
      _ => println!("It's a default vehicle."),
  }
}