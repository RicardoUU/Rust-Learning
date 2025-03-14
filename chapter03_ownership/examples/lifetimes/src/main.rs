fn main() {
    println!("生命周期示例");
    
    // 基本的生命周期标注
    basic_lifetime_annotation();
    
    // 结构体中的生命周期
    struct_lifetime();
    
    // 生命周期省略规则
    lifetime_elision();
    
    // 静态生命周期
    static_lifetime();
}

// 基本的生命周期标注示例
fn basic_lifetime_annotation() {
    println!("\n--- 基本的生命周期标注 ---");
    
    let string1 = String::from("长字符串是长的");
    let string2 = String::from("xyz");
    
    // 调用带有生命周期标注的函数
    let result = longest(string1.as_str(), string2.as_str());
    println!("最长的字符串是: {}", result);
    
    // 生命周期与作用域
    let string1 = String::from("长字符串");
    {
        let string2 = String::from("短");
        let result = longest(string1.as_str(), string2.as_str());
        println!("内部作用域 - 最长的字符串是: {}", result);
        // 这里 result 仍然有效，因为 string1 和 string2 都还在作用域内
    }
    // 这里 string2 已经离开作用域
}

// 带有生命周期标注的函数
// 'a 是生命周期参数，表示 x 和 y 的引用以及返回值的引用至少与 'a 存活一样长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期示例
fn struct_lifetime() {
    println!("\n--- 结构体中的生命周期 ---");
    
    // 定义一个包含引用的结构体，需要生命周期标注
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("从前有一个村庄。很久很久以前...");
    let first_sentence = novel.split('.').next().unwrap();
    
    // 创建结构体实例，其中的引用指向 novel 的一部分
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {}", excerpt.part);
}

// 生命周期省略规则示例
fn lifetime_elision() {
    println!("\n--- 生命周期省略规则 ---");
    
    // 以下函数不需要显式的生命周期标注，因为编译器应用了省略规则
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 省略规则解释：
    // 1. 每个引用参数都有自己的生命周期参数
    // 2. 如果只有一个输入生命周期参数，那么它被赋给所有输出生命周期参数
    // 3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    //    那么 self 的生命周期被赋给所有输出生命周期参数
}

// 静态生命周期示例
fn static_lifetime() {
    println!("\n--- 静态生命周期 ---");
    
    // 'static 生命周期存活于整个程序期间
    let s: &'static str = "我有静态生命周期";
    println!("{}", s);
    
    // 字符串字面量都有 'static 生命周期
    let s2 = "Hello, world!";
    println!("{}", s2);
}