# 第三章：所有权系统

本章节将介绍 Rust 的核心特性：所有权系统，包括所有权规则、引用与借用、以及生命周期概念。

## 3.1 所有权规则

Rust 的所有权系统是其最独特的特性，它使 Rust 能够在不需要垃圾回收的情况下保证内存安全：

1. Rust 中的每一个值都有一个被称为其**所有者**的变量
2. 值在任一时刻有且只有一个所有者
3. 当所有者离开作用域，这个值将被丢弃

```rust
fn main() {
    // s 不可用，它尚未声明
    {
        let s = String::from("hello"); // s 从此处开始可用
        // 使用 s
    }                                  // 此作用域已结束，s 不再可用
}
```

### 移动（Move）

当我们将一个变量赋值给另一个变量时，数据的所有权会发生转移：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    
    // println!("s1: {}", s1); // 错误：s1 的值已被移动
    println!("s2: {}", s2); // 正确
}
```

### 克隆（Clone）

如果我们想要深度复制堆上的数据，可以使用 `clone` 方法：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深度复制
    
    println!("s1: {}", s1); // 正确
    println!("s2: {}", s2); // 正确
}
```

### 栈上数据的复制

实现了 `Copy` trait 的类型在赋值时会进行复制而非移动：

```rust
fn main() {
    let x = 5;
    let y = x; // x 的值被复制给 y
    
    println!("x: {}", x); // 正确
    println!("y: {}", y); // 正确
}
```

## 3.2 引用与借用

引用允许我们使用值但不获取其所有权：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递 s1 的引用
    
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然可用
}

fn calculate_length(s: &String) -> usize {
    s.len() // 返回字符串的长度
    // s 离开作用域，但因为它没有所有权，所以什么也不会发生
}
```

### 可变引用

可变引用允许我们修改借用的值：

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("s: {}", s); // 输出 "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 引用规则

1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须总是有效的

## 3.3 切片类型

切片是对集合中部分连续元素的引用：

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // 或 &s[..5]
    let world = &s[6..11]; // 或 &s[6..]
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    
    // 字符串字面量就是切片
    let s = "Hello, world!"; // s: &str
}
```

## 3.4 生命周期

生命周期是 Rust 中另一种泛型，它确保引用在我们使用它的时候保持有效：

```rust
// 'a 是生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
}
```

## 本章示例

- [所有权示例](./examples/ownership/) - 展示所有权规则和移动语义
- [引用与借用](./examples/references/) - 展示引用和借用的使用
- [切片类型](./examples/slices/) - 展示字符串和数组切片
- [生命周期](./examples/lifetimes/) - 展示生命周期标注的使用

## 练习

1. 编写一个函数，接受一个字符串的所有权，然后返回该字符串
2. 编写一个函数，接受一个字符串的引用，并返回其中的第一个单词
3. 实现一个函数，使用生命周期标注，返回两个字符串切片中较短的那个