fn main() {
    let mut s = String::from("runoob");
    // let slice = &s[0..3];
    s = String::from("rust"); // 错误，被引用的场景不允许重新赋值
    // s.push_str("yes!"); // 错误
    println!("s = {}", s);
    // println!("slice = {}, s = {}", slice, s);
}