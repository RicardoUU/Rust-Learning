# 第八章：泛型、特质和生命周期

本章节将介绍 Rust 中的泛型、特质（Traits）和生命周期，这些是 Rust 类型系统的核心组成部分。

## 8.1 泛型

泛型允许我们编写适用于多种类型的代码，而不必为每种类型编写重复的代码。

### 函数中的泛型

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
}
```

### 结构体中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 不同类型的泛型参数
struct Point2<T, U> {
    x: T,
    y: U,
}
```

### 枚举中的泛型

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 方法中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## 8.2 特质（Traits）

特质定义了一组可以被共享的行为，类似于其他语言中的接口。

### 定义特质

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // 带有默认实现的方法
    fn default_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}
```

### 为类型实现特质

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### 特质作为参数

```rust
// 接受任何实现了 Summary 特质的类型
pub fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}
```

### 特质约束

```rust
// 使用特质约束语法
pub fn notify<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 多个特质约束
pub fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// 使用 where 子句简化复杂约束
pub fn notify<T, U>(t: &T, u: &U) -> String
    where T: Summary + Clone,
          U: Clone + Debug
{
    // ...
}
```

### 返回实现特质的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，你知道的，大家都..."),
        reply: false,
        retweet: false,
    }
}
```

### 使用特质约束有条件地实现方法

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只为那些实现了 Display 和 PartialOrd 特质的类型实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}
```

## 8.3 生命周期

生命周期是 Rust 的另一种泛型，它确保引用在我们使用它的时候保持有效。

### 生命周期注解语法

```rust
&i32        // 一个引用
&'a i32     // 一个带有显式生命周期的引用
&'a mut i32 // 一个带有显式生命周期的可变引用
```

### 函数签名中的生命周期注解

```rust
// 指定参数和返回值的生命周期关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 结构体中的生命周期

```rust
// 结构体包含引用时必须指定生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### 生命周期省略规则

Rust 编译器使用三条规则来推断引用的生命周期：

1. 每个引用参数都有自己的生命周期参数
2. 如果只有一个输入生命周期参数，那么它被赋给所有输出生命周期参数
3. 如果有多个输入生命周期参数，但其中一个是 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋给所有输出生命周期参数

### 静态生命周期

```rust
// 'static 生命周期存活于整个程序运行期间
let s: &'static str = "我有静态生命周期";
```

### 结合泛型类型参数、特质约束和生命周期

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 总结

泛型、特质和生命周期是 Rust 类型系统的核心组成部分，它们共同提供了强大的抽象能力，同时保持了 Rust 的安全性和性能。通过这些特性，我们可以编写灵活、可重用且类型安全的代码。