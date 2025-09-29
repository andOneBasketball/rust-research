/*
给定一组整数，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（出现次数最多的值；在这里哈希 map 会很有帮助）
*/
use std::collections::HashMap;

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 2, 5, 9];
    let mut scores = HashMap::new();
    nums.sort();
    let mid = nums.len() / 2;
    let median = if nums.len() % 2 == 0 {
        (nums[mid - 1] as f64 + nums[mid] as f64) / 2.0
    } else {
        nums[mid] as f64
    };
    println!("Median: {median}");
    for num in nums {
        let count = scores.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut majority = vec![];
    for (num, score) in scores {
        if score > max_count {
            max_count = score;
            majority = vec![num];
        } else if score == max_count {
            majority.push(num);
        }
    }
    println!("Majority: {:?}", majority);
}
