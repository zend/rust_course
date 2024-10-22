fn main() {
  let condition = true;
  let x = if condition { 5 } else { 6 };
  assert_eq!(x, 5);

  for i in 1..=6 {
      println!("{} ", i)
  }

  let mut a: [String; 10] = std::array::from_fn(|i| format!("str{}", i));
  // 传入引用，尝试去掉引用符号看看
  for i in &a {
      println!("{}", i);
  }
  // 不可变借用
  for i in a.iter() {
    println!("{}", i);
  }
  // 如果想要索引值
  for (i, v) in a.iter().enumerate() {
    println!("a[{}] = {}", i, v);
  }
  // 循环修改
  for i in &mut a {
    i.push_str("!");
  }
  // for 循环必须使用引用，这里才可以继续使用
  println!("After loop: {:?}", a);

  let mut n = 0;
  loop {
    if n > 5 {
      break;
    }
    println!("n = {}", n);
    n += 1;
  }
  println!("loop end");

  let mut n = 0;
  while n < 5 {
    println!("while n={}", n);
    n += 1;
  }
}
