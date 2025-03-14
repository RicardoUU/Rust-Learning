# 第13章：Rust 高级特性

本章将探索 Rust 的一些高级特性，这些特性在特定场景下非常有用，但通常不会在日常编程中频繁使用。了解这些特性将帮助你更全面地掌握 Rust 语言。

## 本章内容

- 不安全 Rust：使用 `unsafe` 关键字执行低级操作
- 高级特征：关联类型、默认类型参数、完全限定语法等
- 高级类型：类型别名、never 类型、动态大小类型等
- 高级函数和闭包：函数指针和返回闭包
- 宏：声明式宏和过程宏

## 不安全 Rust

Rust 的安全保证通常由编译器强制执行，但有时需要执行编译器无法验证安全性的操作。这时可以使用 `unsafe` 关键字：

```rust
fn main() {
    let mut num = 5;
    
    // 创建指向数据的裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // 在 unsafe 块中解引用裸指针
    unsafe {
        println!("r1 指向的值: {}", *r1);
        *r2 = 10;
        println!("修改后的值: {}", *r1);
    }
    
    // 调用不安全函数
    unsafe {
        dangerous();
    }
}

// 不安全函数
unsafe fn dangerous() {
    println!("这是一个不安全函数");
}
```

## 高级特征

### 关联类型

关联类型是特征定义中的类型占位符：

```rust
pub trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

### 默认泛型类型参数

```rust
trait Add<RHS=Self> {  // RHS 有默认类型 Self
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

### 运算符重载

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    
    println!("p3 = {:?}", p3);
}
```

## 高级类型

### 类型别名

```rust
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);
}
```

### Never 类型

`!` 类型表示永不返回的计算：

```rust
fn main() {
    let guess: u32 = match "4".parse() {
        Ok(num) => num,
        Err(_) => continue,  // continue 的类型是 !
    };
}
```

## 高级函数和闭包

### 函数指针

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("答案是: {}", answer);  // 12
}
```

### 返回闭包

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## 宏

### 声明式宏

```rust
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
}

fn main() {
    say_hello!();
}
```

## 示例代码

本章包含以下示例：

1. [不安全 Rust](./examples/unsafe_rust/): 裸指针和不安全函数
2. [高级特征](./examples/advanced_traits/): 关联类型和运算符重载
3. [高级类型](./examples/advanced_types/): 类型别名和 never 类型
4. [宏](./examples/macros/): 声明式宏和过程宏

## 练习

1. 实现一个自定义的智能指针类型
2. 创建一个使用运算符重载的复数类型
3. 编写一个简单的声明式宏

## 进一步学习

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/): 不安全 Rust 的指南
- [The Rust Reference](https://doc.rust-lang.org/reference/): Rust 语言的详细参考
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/): 通过示例学习 Rust