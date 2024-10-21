fn main() {
  // 借用
  let s1 = String::from("Hello");
  let s2 = s1; // s1不再可用
  println!("s2={}", s2);

  // 深拷贝
  let s1 = String::from("Deep Copy");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);

  // 浅拷贝：只支持栈变量
  let n1 = 5;
  let n2 = n1;
  println!("n1 = {}, n2 = {}", n1, n2);

  // 函数调用
  let s1 = String::from("Hello");
  take_ownership(s1);
  // s1 不再可用
  // println!("{}", s1);

  let s1 = String::from("Hello");
  let s2 = take_ownership_and_return(s1);
  take_ownership_mutable(s2);

  // 引用与解引用
  ref_and_deref();

  // 只读传递
  readonly_ref();
}

fn take_ownership(x: String) {
  println!("x = {}", x);
}

fn take_ownership_and_return(x: String) -> String {
  println!("x = {}", x);
  x
}

fn take_ownership_mutable(mut x: String) {
  println!("x = {}", x);
  x.push_str(" world!");
  println!("After mutate: {}", x);
}

fn ref_and_deref() {
  let x = 5;
  let y = &x;

  println!("{} {}", x, y);
  assert_eq!(x, *y);
}

fn readonly_ref() {
  let s1 = String::from("Hello");

  fn rdonly(s: &String) {
    println!("只读，不转移所有权：{}", s);

    // 不允许修改
    // s = "1111";
  }

  rdonly(&s1);
  println!("s1={}", s1);
}