// 由于 HashMap 并没有包含在 Rust 的 prelude 库中，所以需要手动引入
use std::collections::HashMap;
fn main() {
    // 创建一个HashMap，用于存储学生成绩
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 100);
    println!("Alice's grade is {}", student_grades.get("Alice").unwrap());
    //println!("Alice's grade is {}", student_grades.get("Alice"));
    
    // 创建指定大小的 HashMap，避免频繁的内存分配和拷贝，提升性能。
    let mut student_grades2 = HashMap::with_capacity(3);
    student_grades2.insert("Alice", 100);
    student_grades2.insert("Bob", 99);
    student_grades2.insert("Eve", 59);
    println!("{:?}", student_grades2);
}