use num_traits::real::Real;

fn main() {
  {
    let num_list = vec![12, 32, 51, 10, 48, 11, 97];
    let mut largest = &num_list[0];
    for number in &num_list {
      if number > largest {
        largest = number;
      }
    }

    println!("The largest number in {:?} is {}", num_list, largest);

    let num_list = vec![12, 32, 51, 10, 48, 11, 97, 100, 1, 223, 311, 3, 132];
    let mut largest = &num_list[0];
    for number in &num_list {
      if number > largest {
        largest = number;
      }
    }

    println!("The largest number in {:?} is {}", num_list, largest);
  }

  // 封装成函数
  {
    fn biggest(list: &[i32]) -> &i32 {
      let mut largest = &list[0];
      for item in list {
        if item > largest {
          largest = item;
        }
      }
      return largest;
    }
    let list1 = vec![12, 32, 51, 10, 48, 11, 97];
    let list2= vec![12, 32, 51, 10, 48, 11, 97, 100, 1, 223, 311, 3, 132];
    println!("The largest number in {:?} is {}", list1, biggest(&list1));
    println!("The largest number in {:?} is {}", list2, biggest(&list2));
    // mismatched types
    // let list3 = vec!['A', 'B', 'c', 'X', 'F'];
    // println!("The largest char in {:?} is {}", list3, biggest(&list3));
    // let list4: Vec<u64> = vec![12, 32, 51, 10, 48, 11, 97, 100, 1, 223, 311, 3, 132];
    // println!("The largest number in {:?} is {}", list4, biggest(&list4));
  }

  {
    fn biggest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list {
        if item > largest {
          largest = item;
        }
      }
      return largest;
    }
    let list1 = vec![12, 32, 51, 10, 48, 11, 97];
    let list2= vec![12, 32, 51, 10, 48, 11, 97, 100, 1, 223, 311, 3, 132];
    let list3 = vec!['A', 'B', 'Z', 'X', 'F'];
    let list4: Vec<u64> = vec![12, 32, 51, 10, 48, 11, 97, 100, 1, 223, 311, 3, 132];
    println!("The largest number in {:?} is {}", list1, biggest(&list1));
    println!("The largest number in {:?} is {}", list2, biggest(&list2));
    println!("The largest number in {:?} is {}", list3, biggest(&list3));
    println!("The largest number in {:?} is {}", list4, biggest(&list4));
  }

  {
    struct Point<T> {
      x: T,
      y: T,
    }
    impl<T: Real> Point<T> {
      fn distance_from_origin(&self) -> T {
        let area = self.x*self.x + self.y*self.y;
        area.sqrt()
      }
    }
    let two_int = Point { x: 1, y: 2 };
    let _two_float = Point { x: 1.0, y: 2.0 };
    let mix = Point { x: 1f32, y: 2.0 };
    println!("x={}, y={}", two_int.x, two_int.y);
    println!("mix {}, {}", mix.x, mix.y);

    let two_float: Point<f32> = Point { x: 3.0, y: 4.0 };
    println!("distance from origin {}", two_float.distance_from_origin());

    struct Point2<T, U> {
      x: T,
      y: U,
    }
    let _two_int = Point2 { x: 1, y: 2 };
    let _two_float = Point2 { x: 1.0, y: 2.0 };
    let mix = Point2 { x: 1, y: 2.0 }; // Ok
    println!("mix {} {}", mix.x, mix.y);
  }
}