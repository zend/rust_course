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
}
