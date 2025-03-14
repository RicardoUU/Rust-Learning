# 第六章：常见集合

本章节将介绍 Rust 标准库中的常见集合类型，这些类型用于存储多个值。

## 6.1 集合类型概述

Rust 标准库包含多种有用的集合数据结构，与内置的数组和元组不同，这些集合存储在堆上，可以在运行时增长或缩小。主要的集合类型包括：

- **Vector**：可变大小的数组
- **String**：UTF-8 编码的文本字符串
- **HashMap**：键值对的哈希表

## 6.2 Vector

Vector 允许你在一个单独的数据结构中存储多个相同类型的值，并在内存中彼此相邻排列。

### 创建 Vector

```rust
// 创建空 Vector
let v: Vec<i32> = Vec::new();

// 使用宏创建 Vector
let v = vec![1, 2, 3];
```

### 更新 Vector

```rust
// 添加元素
let mut v = Vec::new();
v.push(5);
v.push(6);

// 删除元素
let last = v.pop();  // 返回 Some(6)
```

### 读取 Vector 元素

```rust
let v = vec![1, 2, 3, 4, 5];

// 使用索引（可能导致程序崩溃）
let third = &v[2];

// 使用 get 方法（返回 Option）
let third = v.get(2);  // 返回 Some(&3)
let non_existent = v.get(100);  // 返回 None
```

### 遍历 Vector

```rust
let v = vec![100, 32, 57];

// 不可变遍历
for i in &v {
    println!("{}", i);
}

// 可变遍历
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### 使用枚举存储多种类型

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## 6.3 String

String 是 Rust 标准库提供的可增长、可变、有所有权的 UTF-8 编码字符串类型。

### 创建字符串

```rust
// 创建空字符串
let s = String::new();

// 从字符串字面值创建
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

### 更新字符串

```rust
// 追加字符串
let mut s = String::from("foo");
s.push_str("bar");  // s 现在是 "foobar"

// 追加单个字符
s.push('!');  // s 现在是 "foobar!"

// 使用 + 运算符连接字符串
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 被移动，不能再使用

// 使用 format! 宏
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);  // s 是 "tic-tac-toe"
```

### 字符串索引

Rust 不支持通过索引访问 String 中的字符：

```rust
let s = String::from("hello");
let h = s[0];  // 错误！Rust 不允许这样做
```

### 字符串切片

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];  // s 是 "Зд"，注意这是按字节切片，不是字符
```

### 遍历字符串

```rust
// 遍历字符
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// 遍历字节
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

## 6.4 HashMap

HashMap 是一个键值对的集合，通过哈希函数组织数据。

### 创建 HashMap

```rust
use std::collections::HashMap;

// 创建空 HashMap
let mut scores = HashMap::new();

// 插入键值对
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 从两个 Vector 创建
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### 访问 HashMap 中的值

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// 使用 get 方法
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // 返回 Some(&10)

// 遍历 HashMap
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 更新 HashMap

```rust
// 覆盖值
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // 现在值是 25

// 只在键不存在时插入
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);  // Blue 仍然是 25

// 基于旧值更新
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

## 6.5 性能考虑

- Vector：在末尾添加元素很快，在中间插入可能需要移动元素
- String：连接操作可能导致多次分配
- HashMap：默认使用 SipHash 哈希函数，安全但相对较慢

## 练习

1. 实现一个程序，使用 Vector 存储整数列表并计算统计值（平均值、中位数、众数）
2. 将字符串转换为 Pig Latin
3. 使用 HashMap 和 Vector 创建一个简单的员工管理系统