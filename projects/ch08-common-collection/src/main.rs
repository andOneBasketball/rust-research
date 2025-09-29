use std::collections::HashMap;

fn main() {
    println!("===== Vec Demo =====");
    let mut nums = vec![3, 1, 4];
    nums.push(1);
    nums.push(5);
    println!("原始 Vec: {:?}", nums);

    nums.sort();
    println!("排序后: {:?}", nums);

    nums.sort_by(|a, b| b.cmp(a));
    println!("逆序排序后: {:?}", nums);

    if let Some(last) = nums.pop() {
        println!("pop 出来的元素: {}", last);
    }
    println!("剩余 Vec: {:?}", nums);

    for (i, v) in nums.iter().enumerate() {
        println!("索引 {} -> 值 {}", i, v);
    }

    println!("\n===== String Demo =====");
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("拼接后的字符串: {}", s);

    println!("字符串长度: {}", s.len());
    println!("包含 'World'? {}", s.contains("World"));

    let replaced = s.replace("World", "Rust");
    println!("替换结果: {}", replaced);

    let hello = &s[0..5]; // 注意：必须在 UTF-8 边界切分
    println!("切片: {}", hello);

    println!("\n===== HashMap Demo =====");
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("初始 HashMap: {:?}", scores);

    // 查询
    let name = "Alice";
    match scores.get(name) {
        Some(score) => println!("{} 的分数是 {}", name, score),
        None => println!("{} 不存在", name),
    }

    // 更新：覆盖已有值
    scores.insert(String::from("Alice"), 15);
    println!("更新 Alice 后: {:?}", scores);

    // 更新：使用 entry
    scores.entry(String::from("Charlie")).or_insert(30);
    println!("插入 Charlie 后: {:?}", scores);

    // 遍历
    for (k, v) in &scores {
        println!("{} => {}", k, v);
    }
}
