# 第二章：控制流

本章节将介绍 Rust 中的控制流结构，包括条件语句、循环和模式匹配。

## 2.1 if 表达式

Rust 中的 if 表达式允许根据条件执行不同的代码块：

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("数字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("数字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("数字可以被 2 整除");
    } else {
        println!("数字不能被 4、3 或 2 整除");
    }
}
```

在 Rust 中，if 是一个表达式，这意味着它可以返回值：

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

println!("number 的值是: {}", number);
```

## 2.2 循环

Rust 提供了三种循环：`loop`、`while` 和 `for`。

### loop 循环

`loop` 关键字创建一个无限循环，可以使用 `break` 退出：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("结果是: {}", result);
}
```

### while 循环

`while` 循环在条件为真时执行代码块：

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("发射！");
}
```

### for 循环

`for` 循环用于遍历集合中的元素：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("值为: {}", element);
    }
    
    // 使用范围
    for number in 1..4 {
        println!("数字: {}", number);
    }
}
```

## 2.3 match 表达式

`match` 是 Rust 中强大的模式匹配工具：

```rust
fn main() {
    let number = 13;
    
    match number {
        // 匹配单个值
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        // 匹配范围
        4..=10 => println!("四到十之间"),
        // 捕获其他所有值
        _ => println!("其他数字"),
    }
}
```

## 2.4 if let 简洁控制流

`if let` 语法让我们以更简洁的方式处理只匹配一个模式的值：

```rust
fn main() {
    let some_value = Some(3);
    
    // 使用 match
    match some_value {
        Some(3) => println!("是三！"),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(3) = some_value {
        println!("是三！");
    }
}
```

## 本章示例

- [条件语句](./examples/conditionals/) - 展示 if 和 if let 的使用
- [循环示例](./examples/loops/) - 展示各种循环的使用
- [模式匹配](./examples/pattern_matching/) - 展示 match 表达式的使用

## 练习

1. 编写一个程序，使用 if 表达式根据学生的分数（0-100）输出对应的等级（A、B、C、D、F）
2. 创建一个程序，使用 for 循环计算 1 到 100 之间所有偶数的和
3. 编写一个程序，使用 match 表达式处理不同的命令输入（如 "help"、"quit"、"run" 等）