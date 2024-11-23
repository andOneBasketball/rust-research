fn main() {
    let condition = true;
    // 1、if分支赋值，if 语句块是表达式，可以用 if 表达式的返回值来给变量赋值
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number); // print 5

    // 2、循环的跳过和中断
    for item in 1..5 {
        if item == 2 {
            // 跳过本次循环，进入下次循环
            continue;
        }

        if item == 4 {
            // 中断整个循环
            break;
        }
        println!("this Item is : {}", item);
    }

    // 3、For循环中发生所有权转移
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    for item in vec1.into_iter() {
        println!("Item: {}", item);
        // 这里可以对item进行任何操作，因为所有权已经移动到循环中
    }
    // 此处打印 vec1 变量失败，因为所有权已转移给 for 循环
    // println!("{:?}", vec1)

    // 4、For循环借用集合元素：不可变借用
    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    // 通过 &vec2 发生所有权借用
    for item in &vec2 {
        println!("Item: {}!", item);
    }
    // 这里 vec2 依然拥有所有权
    println!("{:?}", vec2);

    // 5、while vs for 循环
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    // while循环中通过指定索引来访问元素，可能存在越界的风险
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // for循环中通过迭代器遍历元素，并不会使用索引去访问数组，不存在越界风险
    // 同时也避免了运行时的边界检查，性能更高。
    for element in a.iter() {
        println!("the value is: {} ！", element);
    }

    for element in &a {
        println!("the value is: {} ！!", *element);
    }

    // 6、loop循环作为表达式
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 满足条件后 break 会中断循环，并返回 counter * 2 的值
            break counter * 2
        }
    };
    println!("The result is {}", result);
}