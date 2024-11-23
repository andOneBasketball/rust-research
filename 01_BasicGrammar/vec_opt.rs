// 主函数
fn main() {
    // 定义一个包含整数的数组
    let mut numbers = vec![1, 2, 3, 4, 5];

    // 使用迭代器对数组进行遍历，并输出每个元素
    println!("Iterating through the array:");
    for num in numbers.iter() {
        println!("{}", num);
    }

    // 使用迭代器的 map 方法对数组中的每个元素进行平方运算，并收集结果到一个新的数组中
    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    // 输出平方后的数组
    println!("Squared numbers: {:?}", squared_numbers);


    // 定义一个包含整数的数组
    numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用迭代器的 filter 方法对数组进行过滤，筛选出偶数
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

    // 输出筛选后的结果
    println!("Even numbers: {:?}", even_numbers);


    // 1.显式声明动态数组类型
    let v1: Vec<i32> = Vec::new();

    // 2.编译器根据元素自动推断类型，须将 v 声明为 mut 后，才能进行修改。
    let mut v2 = Vec::new();
    v2.push(1);

    // 3.使用宏 vec! 来创建数组，支持在创建时就给予初始化值
    let v3 = vec![1, 2, 3];

    // 4.使用 [初始值;长度] 来创建数组，默认值为 0，初始长度为 3
    let v4 = vec![0; 3];  // v4 = [0, 0, 0];

    // 5.使用from语法创建数组
    let v5 = Vec::from([0, 0, 0]);
    assert_eq!(v4, v5);

    let mut v3 = Vec::from([1, 2, 3, 4, 5]);
    for i in &mut v3 {
        *i += 1;
    }
    assert_eq!(v3, vec![2, 3, 4, 5, 6]);
}