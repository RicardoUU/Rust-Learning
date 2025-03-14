fn main() {
    println!("引用与借用示例");
    
    // 基本的引用
    basic_references();
    
    // 可变引用
    mutable_references();
    
    // 引用规则
    reference_rules();
    
    // 悬垂引用
    // dangling_references();
}

// 基本的引用示例
fn basic_references() {
    println!("\n--- 基本的引用 ---");
    
    let s1 = String::from("hello");
    
    // 创建一个指向 s1 的引用，但不获取所有权
    let len = calculate_length(&s1);
    
    // 原始变量 s1 在函数调用后仍然可用
    println!("'{}' 的长度是 {}", s1, len);
}

// 接受字符串引用的函数
fn calculate_length(s: &String) -> usize {
    s.len() // 返回字符串的长度
    // 这里 s 离开作用域，但因为它没有所有权，所以什么也不会发生
}

// 可变引用示例
fn mutable_references() {
    println!("\n--- 可变引用 ---");
    
    let mut s = String::from("hello");
    println!("原始字符串: {}", s);
    
    // 传递可变引用
    change(&mut s);
    println!("修改后的字符串: {}", s);
    
    // 可变引用的限制
    let mut s2 = String::from("hello");
    
    // 在同一作用域中，只能有一个可变引用
    let r1 = &mut s2;
    // 下面的代码会导致编译错误
    // let r2 = &mut s2;
    
    println!("可变引用 r1: {}", r1);
}

// 接受可变引用的函数
fn change(s: &mut String) {
    s.push_str(", world"); // 修改借用的值
}

// 引用规则示例
fn reference_rules() {
    println!("\n--- 引用规则 ---");
    
    let mut s = String::from("hello");
    
    // 规则1：在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    
    // 多个不可变引用是允许的
    let r1 = &s;
    let r2 = &s;
    println!("多个不可变引用: {} {}", r1, r2);
    
    // 不可变引用的作用域结束后，可以创建可变引用
    let r3 = &mut s;
    println!("可变引用: {}", r3);
    
    // 引用的作用域
    let mut s2 = String::from("hello");
    
    let r1 = &s2; // 没问题
    let r2 = &s2; // 没问题
    println!("r1 和 r2: {} {}", r1, r2);
    // r1 和 r2 在这里不再使用
    
    let r3 = &mut s2; // 没问题，因为 r1 和 r2 不再使用
    println!("r3: {}", r3);
}

// 悬垂引用示例（会导致编译错误）
/*
fn dangling_references() {
    println!("\n--- 悬垂引用 ---");
    
    // 以下代码会导致编译错误
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

// 这个函数试图返回一个悬垂引用，但 Rust 编译器会阻止这种情况
fn dangle() -> &String { // 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃，其内存被释放，引用将指向无效的内存
*/