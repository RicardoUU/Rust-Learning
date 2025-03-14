// HashMap示例：展示HashMap的基本操作

use std::collections::HashMap;

fn main() {
    // 创建HashMap的不同方式
    println!("创建HashMap的不同方式:");
    
    // 创建空HashMap
    let mut scores = HashMap::new();
    println!("空HashMap: {:?}", scores);
    
    // 插入键值对
    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("黄队"), 50);
    println!("插入键值对后: {:?}", scores);
    
    // 从两个vector创建HashMap
    println!("\n从两个vector创建HashMap:");
    let teams = vec![String::from("红队"), String::from("绿队")];
    let initial_scores = vec![25, 55];
    
    // 使用zip和collect创建HashMap
    let team_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从vector创建: {:?}", team_scores);
    
    // 访问HashMap中的值
    println!("\n访问HashMap中的值:");
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        String::from("Rust编程语言"),
        String::from("非常好的入门书籍")
    );
    book_reviews.insert(
        String::from("深入理解计算机系统"),
        String::from("经典的计算机科学书籍")
    );
    
    // 使用get方法
    let book_name = String::from("Rust编程语言");
    match book_reviews.get(&book_name) {
        Some(review) => println!("《{}》的评价: {}", book_name, review),
        None => println!("没有找到《{}》的评价", book_name),
    }
    
    // 尝试访问不存在的键
    let nonexistent = String::from("Rust高级编程");
    match book_reviews.get(&nonexistent) {
        Some(review) => println!("《{}》的评价: {}", nonexistent, review),
        None => println!("没有找到《{}》的评价", nonexistent),
    }
    
    // 遍历HashMap
    println!("\n遍历HashMap:");
    for (key, value) in &book_reviews {
        println!("《{}》: {}", key, value);
    }
    
    // 更新HashMap
    println!("\n更新HashMap:");
    let mut word_counts = HashMap::new();
    
    // 插入新值
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        // 使用entry和or_insert的组合来计数
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", word_counts);
    
    // 根据旧值更新
    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);
    
    // 覆盖已有的值
    println!("更新前: {:?}", scores);
    scores.insert(String::from("蓝队"), 25);
    println!("覆盖后: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("黄队")).or_insert(50);
    scores.entry(String::from("蓝队")).or_insert(100); // 不会改变蓝队的分数
    println!("条件插入后: {:?}", scores);
    
    // 删除键值对
    println!("\n删除键值对:");
    let mut fruits = HashMap::new();
    fruits.insert(String::from("苹果"), 5);
    fruits.insert(String::from("香蕉"), 8);
    fruits.insert(String::from("橙子"), 3);
    println!("删除前: {:?}", fruits);
    
    // 删除键值对
    fruits.remove(&String::from("香蕉"));
    println!("删除后: {:?}", fruits);
    
    // HashMap和所有权
    println!("\nHashMap和所有权:");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 此处field_name和field_value已被移动，不能再使用
    println!("HashMap: {:?}", map);
    
    // 如果类型实现了Copy trait，则值会被复制而不是移动
    let mut scores = HashMap::new();
    scores.insert("Blue", 10); // &str实现了Copy
    println!("使用实现Copy的类型: {:?}", scores);
}