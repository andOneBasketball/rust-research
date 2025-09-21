use std::thread;
use std::time::Duration;

// 这个函数耗时3秒
fn get_calculate_result() -> bool {
		// 模拟复杂计算，耗时3s
		thread::sleep(Duration::from_secs(3));
    println!("called this function");
    false
}

fn main() {
    // 打印各国语言的单个字符
    let thai_char  = 'ก';
    let korean_char = '한';
    let traditional_chinese_char = '繁';
    let indonesian_char = 'ä';
    // 注意，这里str是字符串类型，不是字符，只不过长度为1
    let str = "国";
    println!("thai_char : {}", thai_char );
    println!("Korean: {}", korean_char);
    println!("Traditional Chinese: {}", traditional_chinese_char);
    println!("Indonesian: {}", indonesian_char);
    
    //测测你和我中间有多少个字符
    for i in '你'..='我' {
        print!("{}", i);//你佡佢佣……戏成我，中间共有4786个字符
    }
    
    let f: bool = true;
    // 触发短路原则，不会调用get_calculate_result函数进行复杂计算
    // 如果改成 get_calculate_result() | f，则会先调用函数，有性能影响
    if f || get_calculate_result() {
        println!("Success!");
    }    
} 