fn main() {
    let mut s = String::from("hello");

    println!("Before mutation: {s}");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{r1} and {r2}");
    // 此位置之后 r1 和 r2 不再使用
    println!("{r1}");

    let r3 = &mut s; // 没问题
    println!("{r3}");
}
