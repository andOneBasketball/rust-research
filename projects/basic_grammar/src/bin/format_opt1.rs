fn main() {
    // 1. 最基础的用法
    let hello = format!("Hello, {}!", "Rust");
    println!("{}", hello);

    // 2. 命名参数
    let named = format!("{lang} is {adj}", lang = "Rust", adj = "fast");
    println!("{}", named);

    // 3. 对齐与填充
    let right = format!("{:>5}", 42); // 右对齐
    let left = format!("{:<5}", 42); // 左对齐
    let center = format!("{:^5}", 42); // 居中
    let pad = format!("{:0>5}", 42); // 用 0 填充
    println!(
        "right: '{}', left: '{}', center: '{}', pad: '{}'",
        right, left, center, pad
    );

    // 4. 数字格式化
    let num = 255;
    println!(
        "dec: {}, bin: {:b}, oct: {:o}, hex: {:x}, HEX: {:X}, hex: {:#x}",
        num, num, num, num, num, num
    );

    // 5. 浮点数格式化
    let pi = 3.1415926;
    println!("pi default: {}", pi);
    println!("pi 2 decimals: {:.2}", pi);
    println!("pi width=8, 3 decimals: {:8.3}", pi);
    println!("pi scientific: {:e}", pi);

    // 6. Debug 格式化
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 4 };
    println!("Debug: {:?}", p);
    println!("Pretty Debug:\n{:#?}", p);

    // 7. 组合用法
    let name = "Alice";
    let age = 18;
    let s = format!("{name} is {age} years old (hex: {age:#x})");
    println!("{}", s);
}
