fn main() {
    println!("切片类型示例");
    
    // 字符串切片
    string_slices();
    
    // 数组切片
    array_slices();
    
    // 实用的字符串切片示例：查找第一个单词
    first_word_example();
}

// 字符串切片示例
fn string_slices() {
    println!("\n--- 字符串切片 ---");
    
    let s = String::from("hello world");
    
    // 创建字符串的切片
    let hello = &s[0..5]; // 从索引0到索引4（不包括5）
    let world = &s[6..11]; // 从索引6到索引10（不包括11）
    
    println!("完整字符串: {}", s);
    println!("hello 切片: {}", hello);
    println!("world 切片: {}", world);
    
    // 省略起始索引和结束索引
    let slice1 = &s[0..5]; // 等同于 &s[..5]
    let slice2 = &s[6..11]; // 等同于 &s[6..]
    let slice3 = &s[0..s.len()]; // 等同于 &s[..]
    
    println!("省略起始索引: {}", slice1);
    println!("省略结束索引: {}", slice2);
    println!("完整切片: {}", slice3);
    
    // 字符串字面量是切片
    let literal = "Hello, world!"; // 类型是 &str
    println!("字符串字面量（切片）: {}", literal);
}

// 数组切片示例
fn array_slices() {
    println!("\n--- 数组切片 ---");
    
    let a = [1, 2, 3, 4, 5];
    
    // 创建数组的切片
    let slice = &a[1..3]; // 类型是 &[i32]
    
    // 使用 debug 格式打印数组和切片
    println!("完整数组: {:?}", a);
    println!("数组切片: {:?}", slice);
    
    // 遍历切片
    println!("遍历切片中的元素:");
    for element in slice {
        println!("{}", element);
    }
}

// 实用的字符串切片示例：查找第一个单词
fn first_word_example() {
    println!("\n--- 查找第一个单词 ---");
    
    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 也可以直接在字符串字面量上调用
    let s2 = "Rust programming";
    let word2 = first_word(s2);
    println!("第一个单词: {}", word2);
}

// 返回字符串中第一个单词的函数
// 接受 &str 类型，这样可以同时接受 String 和 &str
fn first_word(s: &str) -> &str {
    // 将字符串转换为字节数组
    let bytes = s.as_bytes();
    
    // 使用迭代器和枚举查找空格
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 找到空格，返回第一个单词
            return &s[0..i];
        }
    }
    
    // 没有找到空格，整个字符串就是一个单词
    &s[..]
}