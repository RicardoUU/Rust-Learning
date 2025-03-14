// Vector示例：展示Vector的基本操作

fn main() {
    // 创建Vector的不同方式
    println!("创建Vector的不同方式:");
    
    // 创建空Vector
    let v1: Vec<i32> = Vec::new();
    println!("空Vector: {:?}", v1);
    
    // 使用宏创建Vector
    let v2 = vec![1, 2, 3, 4, 5];
    println!("使用vec!宏创建: {:?}", v2);
    
    // 添加元素
    println!("\n添加元素:");
    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);
    println!("添加元素后: {:?}", v3);
    
    // 读取元素
    println!("\n读取元素:");
    let v4 = vec![10, 20, 30, 40, 50];
    
    // 使用索引（注意：如果索引越界会导致程序崩溃）
    let third = &v4[2];
    println!("使用索引访问第三个元素: {}", third);
    
    // 使用get方法（返回Option，更安全）
    match v4.get(2) {
        Some(value) => println!("使用get方法访问第三个元素: {}", value),
        None => println!("索引超出范围"),
    }
    
    // 尝试访问不存在的元素
    match v4.get(100) {
        Some(value) => println!("第101个元素: {}", value),
        None => println!("索引100超出范围"),
    }
    
    // 遍历Vector
    println!("\n遍历Vector:");
    let v5 = vec![100, 32, 57];
    
    // 不可变遍历
    print!("不可变遍历: ");
    for i in &v5 {
        print!("{} ", i);
    }
    println!();
    
    // 可变遍历并修改元素
    let mut v6 = vec![10, 20, 30];
    print!("修改前: {:?}\n修改后: ", v6);
    for i in &mut v6 {
        *i += 5; // 解引用并修改值
        print!("{} ", i);
    }
    println!();
    
    // 使用枚举存储不同类型
    println!("\n使用枚举存储不同类型:");
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Rust编程")),
    ];
    
    // 处理不同类型的元素
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("整数: {}", value),
            SpreadsheetCell::Float(value) => println!("浮点数: {}", value),
            SpreadsheetCell::Text(value) => println!("文本: {}", value),
        }
    }
}