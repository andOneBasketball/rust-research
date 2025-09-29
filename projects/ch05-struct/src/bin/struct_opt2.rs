use std::fmt; // 导入 fmt 模块，其中定义了 Display trait

struct Circle {
    radius: u32,
    color: String,
}

// 手动实现 Display
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里定义当 println!("{}", circle) 时该如何打印
        write!(
            f,
            "Circle with radius {} and color {}",
            self.radius, self.color
        )
    }
}

fn main() {
    let c = Circle {
        radius: 10,
        color: String::from("red"),
    };

    println!("{}", c); // 自动调用 Display::fmt
    //println!("{:?}", c); // ❌ 编译错误，因为没有 #[derive(Debug)]
}
