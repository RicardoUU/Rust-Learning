# 第七章：错误处理

本章节将介绍 Rust 的错误处理机制，包括可恢复错误和不可恢复错误的处理方式。

## 7.1 错误处理概述

Rust 将错误分为两大类：

- **可恢复错误**：这类错误通常不会导致程序完全停止，可以被处理后继续执行，如文件未找到。Rust 使用 `Result<T, E>` 类型处理可恢复错误。
- **不可恢复错误**：这类错误通常表示程序中的 bug，如尝试访问数组越界。Rust 使用 `panic!` 宏处理不可恢复错误。

## 7.2 使用 panic! 处理不可恢复错误

当 Rust 程序遇到无法处理的情况时，可以使用 `panic!` 宏来终止程序。

### panic! 的工作原理

```rust
fn main() {
    panic!("崩溃并燃烧");
}
```

当执行 `panic!` 宏时，程序会：
1. 打印错误信息
2. 展开并清理栈数据
3. 退出程序

### 使用 panic! 的场景

- 遇到不可恢复的错误
- 代码进入了无效状态
- 外部不可控代码出现错误
- 作为原型或测试时的快速失败机制

### 通过环境变量控制 panic 行为

可以通过设置 `RUST_BACKTRACE` 环境变量来获取详细的调用栈信息：

```bash
RUST_BACKTRACE=1 cargo run
```

## 7.3 使用 Result 处理可恢复错误

`Result` 是一个枚举类型，定义如下：

```rust
enum Result<T, E> {
    Ok(T),  // 操作成功，包含成功值
    Err(E), // 操作失败，包含错误信息
}
```

### 基本用法

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");
    
    let file = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("打开文件时出错: {:?}", error);
            return;
        }
    };
    
    // 使用文件...
}
```

### 匹配不同的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");
    
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件时出错: {:?}", other_error),
        },
    };
}
```

### 错误处理的简写：unwrap 和 expect

```rust
// unwrap: 成功返回值，失败则 panic
let file = File::open("hello.txt").unwrap();

// expect: 类似 unwrap，但可以自定义 panic 消息
let file = File::open("hello.txt").expect("无法打开 hello.txt 文件");
```

## 7.4 错误传播

当函数内部遇到错误时，可以将错误传播给调用者处理。

### 使用 match 传播错误

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

### 使用 ? 运算符简化错误传播

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

更简洁的写法：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### ? 运算符与 from 特征

`?` 运算符不仅可以传播错误，还可以进行错误类型的自动转换，前提是目标错误类型实现了 `From<SourceError>` 特征。

## 7.5 何时使用 panic! 和 Result

### 适合使用 panic! 的场景

- 示例、原型和测试
- 确实无法恢复的情况
- 错误处理逻辑比业务逻辑更复杂时
- 你比编译器更了解某个操作不会失败

### 适合使用 Result 的场景

- 可能会失败的操作
- 调用者需要处理错误的情况
- 库函数中的错误（让调用者决定如何处理）

## 7.6 自定义错误类型

对于复杂应用，通常需要定义自己的错误类型：

```rust
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    InvalidInput(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}
```

## 总结

Rust 的错误处理机制强制开发者显式处理错误情况，这有助于编写更健壮的程序。通过区分可恢复和不可恢复错误，以及提供丰富的错误处理工具，Rust 使错误处理变得既安全又灵活。