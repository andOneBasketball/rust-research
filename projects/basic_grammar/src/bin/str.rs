fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');
    println!("{}", s);
    assert_eq!(s, "hello, world!");
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("{:?}", t);
}