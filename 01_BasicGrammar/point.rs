use std::mem;

// 不可变引用，获取值的长度
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", hackquest.");
}

// 悬垂引用（编译不通过）
/*
fn dangle() -> &String {
    // 创建拥有字符串所有权的变量s
    let s = String::from("hello");

    // 返回对象的借用
    &s
    
} //离开函数体作用域后，变量s的内存空间会被自动释放掉，此时&s就成为无效指针（悬垂引用），因此，
  //会编译失败
*/
fn dangle() -> String {
    // 创建拥有字符串所有权的变量s
    let s = String::from("he");

    // 返回对象的借用
    s
} 

fn main() {
    /*
    pub struct String {
        ptr: *const u8,  // 指向堆内存的指针
        len: usize,      // 字符串长度
        capacity: usize, // 堆内存的总容量
    }
    所以 s1 是 24字节
     */
    let s1 = String::from("hello");

    // &s1 即不可变引用（默认），也就是在函数中我们只能读取对象，而不能修改对象
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}, point address {:p}, point size {}.", s1, len, &s1, mem::size_of_val(&s1));

    let x: i64 = 42;
    println!("Pointer size: {} bytes, address {:p}.", mem::size_of_val(&x), &x);


    let mut s2 = String::from("hi");
    // &mut s2 即可变引用，所以 change 函数可以修改该值
    //let r1: &mut String = &mut s2;
    let r1 = &mut s2;
    change(r1);
    println!("mut s2 {}.", s2);
    // println!("r1 {}.", *r1);
    //println!("mut string r1 {}, s2 {}.", *r1, s2);   // 不能同时使用不可变引用和可变引用

    // 试图访问悬垂引用的对象，编译失败
    let reference_to_nothing = dangle();
    println!("reference_to_nothing {}", reference_to_nothing);
}