// 统计示例：计算整数列表的平均值、中位数和众数

use std::collections::HashMap;

fn main() {
    // 创建一个整数列表
    let numbers = vec![5, 1, 3, 7, 2, 5, 8, 9, 3, 5, 4];
    println!("整数列表: {:?}\n", numbers);
    
    // 计算平均值
    let average = calculate_average(&numbers);
    println!("平均值: {}", average);
    
    // 计算中位数
    let median = calculate_median(&numbers);
    println!("中位数: {}", median);
    
    // 计算众数
    let mode = calculate_mode(&numbers);
    println!("众数: {}", mode);
}

// 计算平均值
fn calculate_average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// 计算中位数
fn calculate_median(numbers: &[i32]) -> f64 {
    // 创建一个可变的副本并排序
    let mut sorted = numbers.to_vec();
    sorted.sort();
    
    let len = sorted.len();
    if len % 2 == 0 {
        // 偶数个元素，取中间两个的平均值
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (sorted[mid_left] + sorted[mid_right]) as f64 / 2.0
    } else {
        // 奇数个元素，直接取中间值
        sorted[len / 2] as f64
    }
}

// 计算众数（出现次数最多的数）
fn calculate_mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();
    
    // 统计每个数字出现的次数
    for &number in numbers {
        let count = occurrences.entry(number).or_insert(0);
        *count += 1;
    }
    
    // 找出出现次数最多的数字
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
        .unwrap()
}