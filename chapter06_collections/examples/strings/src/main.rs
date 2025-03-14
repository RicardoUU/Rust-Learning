// String示例：展示String的基本操作

fn main() {
    // 创建字符串的不同方式
    println!("创建字符串的不同方式:");
    
    // 创建空字符串
    let s1 = String::new();
    println!("空字符串: '{}'", s1);
    
    // 从字符串字面值创建
    let s2 = "初始内容".to_string();
    println!("使用to_string()创建: '{}'", s2);
    
    let s3 = String::from("另一种初始内容");
    println!("使用String::from()创建: '{}'", s3);
    
    // 更新字符串
    println!("\n更新字符串:");
    let mut s4 = String::from("你好");
    s4.push_str("，世界"); // 追加字符串
    println!("追加字符串后: '{}'", s4);
    
    let mut s5 = String::from("你好，世界");
    s5.push('!'); // 追加单个字符
    println!("追加字符后: '{}'", s5);
    
    // 使用+运算符连接字符串
    let s6 = String::from("你好，");
    let s7 = String::from("Rust!");
    let s8 = s6 + &s7; // 注意：s6被移动，不能再使用
    println!("使用+运算符连接: '{}'", s8);
    
    // 使用format!宏连接多个字符串
    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");
    let s12 = format!("{}-{}-{}", s9, s10, s11);
    println!("使用format!宏连接: '{}'", s12);
    
    // 字符串切片
    println!("\n字符串切片:");
    let s13 = String::from("你好世界");
    // 注意：这里按字节切片，不是字符，在处理非ASCII字符时需要小心
    // 由于每个中文字符占3个字节，所以这里切片包含"你好"
    let hello = &s13[0..6];
    println!("切片[0..6]: '{}'", hello);
    
    // 遍历字符串
    println!("\n遍历字符串:");
    let s14 = String::from("नमस्ते"); // 梵文"你好"
    
    // 按字符遍历
    print!("按字符遍历: ");
    for c in s14.chars() {
        print!("{} ", c);
    }
    println!();
    
    // 按字节遍历
    print!("按字节遍历: ");
    for b in s14.bytes() {
        print!("{} ", b);
    }
    println!();
    
    // 字符串操作
    println!("\n字符串操作:");
    let mut s15 = String::from("Hello World");
    println!("原始字符串: '{}'", s15);
    
    // 转换为大写（需要创建新字符串）
    let s16 = s15.to_uppercase();
    println!("转换为大写: '{}'", s16);
    
    // 替换
    s15 = s15.replace("World", "Rust");
    println!("替换后: '{}'", s15);
    
    // 分割
    let parts: Vec<&str> = s15.split(' ').collect();
    println!("分割后: {:?}", parts);
}