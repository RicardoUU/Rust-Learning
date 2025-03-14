// 生命周期示例：展示Rust中生命周期的基本用法

fn main() {
    println!("生命周期示例");
    
    // 基本的生命周期问题
    println!("\n1. 基本的生命周期问题");
    basic_lifetime_example();
    
    // 生命周期注解
    println!("\n2. 生命周期注解");
    lifetime_annotation_example();
    
    // 结构体中的生命周期
    println!("\n3. 结构体中的生命周期");
    struct_lifetime_example();
    
    // 生命周期省略规则
    println!("\n4. 生命周期省略规则");
    lifetime_elision_example();
    
    // 静态生命周期
    println!("\n5. 静态生命周期");
    static_lifetime_example();
    
    // 结合泛型和特质约束的生命周期
    println!("\n6. 结合泛型和特质约束的生命周期");
    combined_example();
}

// 基本的生命周期问题
fn basic_lifetime_example() {
    let r;
    
    {
        let x = 5;
        // 下面的代码会导致编译错误，因为x的生命周期比r短
        // r = &x;
    }
    
    // 这里使用r会导致悬垂引用
    // println!("r: {}", r);
    
    // 正确的用法
    let x = 5;
    r = &x;
    println!("r: {}", r);
}

// 生命周期注解示例
fn lifetime_annotation_example() {
    let string1 = String::from("长字符串是长的");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("最长的字符串是: {}", result);
    
    // 生命周期约束示例
    {
        let string1 = String::from("长字符串");
        {
            let string2 = String::from("短");
            // result的生命周期受到string2的限制
            let result = longest(string1.as_str(), string2.as_str());
            println!("内部作用域 - 最长的字符串是: {}", result);
            // 这里可以使用result，因为string2仍然有效
        }
        // 这里不能使用result，因为string2已经失效
    }
}

// 带有生命周期注解的函数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
fn struct_lifetime_example() {
    let novel = String::from("从前有座山。山上有座庙...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {}", excerpt.part);
    println!("公告: {}", excerpt.announce_and_return_part("重要公告"));
    
    // 使用方法中的不同生命周期
    println!("等级: {}", excerpt.level());
}

// 带有生命周期注解的结构体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 为带有生命周期的结构体实现方法
impl<'a> ImportantExcerpt<'a> {
    // 方法使用相同的生命周期参数
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("请注意: {}", announcement);
        self.part
    }
    
    // 使用不同的生命周期参数
    fn level(&self) -> i32 {
        3
    }
}

// 生命周期省略规则示例
fn lifetime_elision_example() {
    // 规则1: 每个引用参数都有自己的生命周期
    // 规则2: 如果只有一个输入生命周期参数，它被赋给所有输出生命周期参数
    // 规则3: 如果有多个输入生命周期参数，但其中一个是&self或&mut self，
    //       那么self的生命周期被赋给所有输出生命周期参数
    
    let s = String::from("生命周期省略规则");
    
    // 不需要显式生命周期注解的函数
    let first_word = first_word(&s);
    println!("第一个单词: {}", first_word);
    
    // 方法中的生命周期省略
    let excerpt = ImportantExcerpt { part: &s };
    println!("摘录部分: {}", excerpt.part);
}

// 应用生命周期省略规则的函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 静态生命周期示例
fn static_lifetime_example() {
    // 'static生命周期存活于整个程序运行期间
    let s: &'static str = "我有静态生命周期";
    println!("{}", s);
    
    // 字符串字面值总是'static
    let literal = "字符串字面值总是静态的";
    println!("{}", literal);
}

// 结合泛型、特质约束和生命周期
fn combined_example() {
    use std::fmt::Display;
    
    let string1 = String::from("长字符串是长的");
    let string2 = String::from("xyz");
    let announcement = "重要公告";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2.as_str(),
        announcement,
    );
    
    println!("最长的字符串是: {}", result);
}

// 结合泛型、特质约束和生命周期的函数
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("公告！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}