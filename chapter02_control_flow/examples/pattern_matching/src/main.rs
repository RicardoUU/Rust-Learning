fn main() {
    println!("模式匹配示例");
    
    // 基本的match表达式
    basic_match();
    
    // 匹配范围和多种模式
    match_ranges_and_multiple();
    
    // 匹配结构体和元组
    match_structs_and_tuples();
    
    // 使用if let简化匹配
    if_let_examples();
    
    // 在while循环中使用模式匹配
    while_let_example();
    
    // 在函数参数中使用模式匹配
    let point = Point { x: 10, y: 20 };
    print_coordinates(&point);
}

// 基本的match表达式示例
fn basic_match() {
    println!("\n--- 基本的match表达式 ---");
    
    let number = 13;
    
    println!("匹配数字 {}", number);
    match number {
        // 匹配单个值
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        // 匹配范围
        4..=10 => println!("四到十之间"),
        // 捕获其他所有值
        other => println!("其他数字: {}", other),
    }
    
    // 使用Option枚举进行匹配
    let some_value = Some(42);
    match some_value {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
}

// 匹配范围和多种模式
fn match_ranges_and_multiple() {
    println!("\n--- 匹配范围和多种模式 ---");
    
    let x = 'c';
    
    match x {
        'a'..='j' => println!("早期字母"),
        'k'..='z' => println!("后期字母"),
        _ => println!("其他字符"),
    }
    
    let y = 10;
    
    match y {
        1 | 3 | 5 | 7 | 9 => println!("奇数"),
        2 | 4 | 6 | 8 | 10 => println!("偶数"),
        _ => println!("超出范围"),
    }
}

// 定义一个结构体用于匹配示例
struct Point {
    x: i32,
    y: i32,
}

// 匹配结构体和元组
fn match_structs_and_tuples() {
    println!("\n--- 匹配结构体和元组 ---");
    
    // 匹配元组
    let tuple = (1, 2, 3);
    
    match tuple {
        (1, y, z) => println!("第一个元素是1，其余是 {} 和 {}", y, z),
        (x, 2, z) => println!("第二个元素是2，其余是 {} 和 {}", x, z),
        _ => println!("其他情况"),
    }
    
    // 匹配结构体
    let point = Point { x: 0, y: 7 };
    
    match point {
        Point { x: 0, y } => println!("在y轴上，y = {}", y),
        Point { x, y: 0 } => println!("在x轴上，x = {}", x),
        Point { x, y } => println!("在其他位置: ({}, {})", x, y),
    }
}

// 使用if let简化匹配
fn if_let_examples() {
    println!("\n--- if let简化匹配 ---");
    
    let some_value = Some(42);
    
    // 使用match
    match some_value {
        Some(value) => println!("match: 值是 {}", value),
        None => (),
    }
    
    // 使用if let（更简洁）
    if let Some(value) = some_value {
        println!("if let: 值是 {}", value);
    }
    
    // 带else的if let
    let different_value: Option<i32> = None;
    if let Some(value) = different_value {
        println!("有值: {}", value);
    } else {
        println!("没有值");
    }
}

// 在while循环中使用模式匹配
fn while_let_example() {
    println!("\n--- while let示例 ---");
    
    let mut stack = Vec::new();
    
    // 添加一些元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // 使用while let弹出并处理每个元素
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
}

// 在函数参数中使用模式匹配
fn print_coordinates(point: &Point) {
    println!("\n--- 函数参数中的模式匹配 ---");
    println!("当前坐标: ({}, {})", point.x, point.y);
}