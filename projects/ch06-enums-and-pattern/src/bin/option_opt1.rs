fn main() {
    // 1. 创建 Option
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // 2. match 匹配
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    match no_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    // 3. if let 简化匹配
    if let Some(n) = some_number {
        println!("if let got number: {}", n);
    }

    // 4. unwrap / unwrap_or / unwrap_or_else
    println!("unwrap: {}", some_number.unwrap()); // 42
    println!("unwrap_or: {}", no_number.unwrap_or(100)); // 100
    println!("unwrap_or_else: {}", no_number.unwrap_or_else(|| 200)); // 200

    // 5. map 对 Option 内部值操作
    let doubled = some_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled); // Some(84)

    // 6. and_then / flat_map 链式操作
    let plus_one = some_number.and_then(|n| Some(n + 1));
    println!("Plus one: {:?}", plus_one); // Some(43)

    // 7. filter 条件过滤
    let filtered = some_number.filter(|&n| n > 50);
    println!("Filtered (>50): {:?}", filtered); // None

    let filtered2 = some_number.filter(|&n| n > 40);
    println!("Filtered (>40): {:?}", filtered2); // Some(42)

    // 8. 链式组合
    let result = some_number
        .map(|n| n * 3)
        .and_then(|n| if n > 100 { None } else { Some(n) })
        .unwrap_or(0);
    println!("Chained result: {}", result); // 126 -> unwrap_or 0 不会触发
}
