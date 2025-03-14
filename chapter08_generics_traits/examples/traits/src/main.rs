// 特质示例：展示Rust中特质(Traits)的基本用法

// 定义一个特质
pub trait Summary {
    // 必须实现的方法
    fn summarize(&self) -> String;
    
    // 带有默认实现的方法
    fn default_summary(&self) -> String {
        format!("(阅读更多... 作者: {})", self.summarize_author())
    }
    
    // 可以被其他方法调用的特质方法
    fn summarize_author(&self) -> String {
        String::from("佚名")
    }
}

// 定义一些实现特质的类型
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为NewsArticle实现Summary特质
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, 来自 {} ({})", self.headline, self.location, self.author)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// 为Tweet实现Summary特质
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特质作为参数
pub fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}

// 使用特质约束语法
pub fn notify_with_trait_bound<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 使用多个特质约束
use std::fmt::{Display, Debug};

pub fn notify_with_multiple_traits<T: Summary + Display>(item: &T) {
    println!("突发新闻！{} ({})", item.summarize(), item);
}

// 使用where子句简化特质约束
pub fn some_function<T, U>(t: &T, u: &U) -> String
    where T: Display + Clone,
          U: Clone + Debug
{
    format!("T: {}, U: {:?}", t, u)
}

// 返回实现特质的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，你知道的，大家都..."),
        reply: false,
        retweet: false,
    }
}

// 使用特质约束有条件地实现方法
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只为那些实现了Display和PartialOrd特质的类型实现cmp_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是x = {}", self.x);
        } else {
            println!("最大的成员是y = {}", self.y);
        }
    }
}

// 为任何实现了特定特质的类型实现另一个特质（毯子实现）
trait PrintInDebug {
    fn print_in_debug(&self);
}

// 为任何实现了Debug特质的类型实现PrintInDebug特质
impl<T: Debug> PrintInDebug for T {
    fn print_in_debug(&self) {
        println!("Debug视图: {:?}", self);
    }
}

fn main() {
    println!("特质示例");
    
    // 使用实现了特质的类型
    println!("\n1. 使用实现了特质的类型");
    let article = NewsArticle {
        headline: String::from("Rust 1.50发布"),
        location: String::from("中国"),
        author: String::from("Rust团队"),
        content: String::from("Rust 1.50版本带来了许多新特性..."),
    };
    
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("我们刚刚发布了Rust 1.50！"),
        reply: false,
        retweet: true,
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize());
    
    // 使用默认实现
    println!("\n2. 使用默认实现");
    println!("文章默认摘要: {}", article.default_summary());
    println!("推文默认摘要: {}", tweet.default_summary());
    
    // 使用特质作为参数
    println!("\n3. 使用特质作为参数");
    notify(&article);
    notify(&tweet);
    
    // 使用特质约束
    println!("\n4. 使用特质约束");
    notify_with_trait_bound(&article);
    
    // 返回实现特质的类型
    println!("\n5. 返回实现特质的类型");
    let returned_item = returns_summarizable();
    println!("返回的项目: {}", returned_item.summarize());
    
    // 有条件地实现方法
    println!("\n6. 有条件地实现方法");
    let pair = Pair::new(3.5, 2.8);
    pair.cmp_display();
    
    // 毯子实现
    println!("\n7. 毯子实现");
    let numbers = vec![1, 2, 3];
    numbers.print_in_debug();
    "Hello".print_in_debug();
    
    println!("\n特质是Rust中实现多态性的主要方式，它们允许代码共享行为而不需要继承。");
}