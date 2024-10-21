fn main() {
    let (a, mut b) : (bool, bool) = (true, true);
    println!("init a={}, b={}", a, b);
    b = false;

    println!("a={}, b={}", a, b);

    let (c, d) : (i32, &str) = (10, "string");
    println!("c={}, d={}", c, d);

    const DEFAULT_SIZE : u32 = 1000 * 1000;
    println!("Default Size: {}", DEFAULT_SIZE);

    // Shadowing
    {
        println!("访问上层的变量 a={}, b={}", a, b);
        let a = 999;
        let b = 888;
        println!("遮蔽上层的变量，结果： a={}, b={}", a, b);
    }
    println!("遮蔽不会影响外层的变量内容： a={}, b={}", a, b);

    // 同层级也可以遮蔽
    let a = "Hello";
    let b = "World!";
    println!("{} {}", a, b);

    let spaces = "       ";
    let spaces = spaces.len();
    println!("Spaces count is {}", spaces);

    // 基本类型
    // 数字：i8, i16, i32, i64, isize，无符号 u8, u16, u32, u64, usize，浮点数 f32, f64
    let age = "41".parse::<u8>().expect("Not a number");
    println!("Age is {}", age);
    // 各种奇怪写法
    let num_base10 = 34_678;
    let num_base16 = 0xefaa;
    let num_base8 = 0o731;
    let num_base2 = 0b11101001;

    // {} 默认转换成十进制打印，可以使用 x/X/o/b 等转换成相应的进制
    println!("{} {:#X} {:#o} {:#b}", num_base10, num_base16, num_base8, num_base2);

    // 处理溢出
    let a: u8 = 255;
    println!("a={}, a - 10 = {}, wrapping = {}, saturating = {}", 
        a, 
        a - 10, 
        a.wrapping_add(10), 
        a.saturating_add(10));

    
    // 浮点数
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc = ({:#x}, {:#x}, {:#x}, {:#x})", abc.0.to_bits(), abc.1.to_bits(), abc.2.to_bits(), (abc.0 + abc.1).to_bits());
    println!("xyz = ({:#x}, {:#x}, {:#x}, {:#x})", xyz.0.to_bits(), xyz.1.to_bits(), xyz.2.to_bits(), (xyz.0 + xyz.1).to_bits());

    // 范围
    for i in 1..5 {
        println!("i = {}", i);
    }
    for j in 1..=5 {
        println!("j = {}", j);
    }
    for k in 'a'..'g' {
        println!("k = {}", k);
    }

    // String
    let mut s = String::from("你好，");
    s.push_str("世界");
    println!("{}", s);
}
