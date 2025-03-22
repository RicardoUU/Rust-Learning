fn main() {
    // loop 循环示例
    println!("\n--- loop 循环示例 ---");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("计数: {}", counter);

        if counter == 5 {
            break counter * 2;
        }
    };

    println!("loop 结果是: {}", result);

    // while 循环示例
    println!("\n--- while 循环示例 ---");
    let mut number = 3;

    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }

    println!("发射！");

    // for 循环示例
    println!("\n--- for 循环遍历数组 ---");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("值为: {}", element);
    }
    
    // 使用范围的 for 循环
    println!("\n--- for 循环使用范围 ---");
    for number in 1..4 {
        println!("数字: {}", number);
    }
    
    // 计算偶数和示例
    println!("\n--- 计算偶数和示例 ---");
    let mut sum = 0;
    
    for number in 1..=10 {
        if number % 2 == 0 {
            sum += number;
        }
    }
    
    println!("1到10之间的偶数和: {}", sum);
}