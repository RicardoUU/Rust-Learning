fn main() {
    // 基本的 if 表达式
    let number = 6;

    if number % 4 == 0 {
        println!("数字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("数字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("数字可以被 2 整除");
    } else {
        println!("数字不能被 4、3 或 2 整除");
    }

    // if 作为表达式使用
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number 的值是: {}", number);

    // 使用 if let 简化模式匹配
    let some_value = Some(3);
    
    // 使用 match
    match some_value {
        Some(3) => println!("match: 值是三！"),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(3) = some_value {
        println!("if let: 值是三！");
    }

    // 成绩等级示例
    let score = 85;
    let grade = if score >= 90 { "A" }
               else if score >= 80 { "B" }
               else if score >= 70 { "C" }
               else if score >= 60 { "D" }
               else { "F" };
    
    println!("分数 {} 的等级是: {}", score, grade);
}