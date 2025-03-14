# 第一章：Rust 基础入门

本章节将介绍 Rust 编程语言的基础知识，帮助你快速入门这门现代系统编程语言。

## 1.1 Rust 简介

Rust 是一门系统编程语言，专注于安全性、并发性和性能。它的主要特点包括：

- **内存安全**：没有垃圾回收器，但通过所有权系统保证内存安全
- **零成本抽象**：高级语言特性不会带来运行时开销
- **并发安全**：编译时防止数据竞争
- **跨平台**：支持多种操作系统和硬件架构

## 1.2 安装 Rust

### 使用 rustup 安装（推荐）

在大多数平台上，可以使用 rustup 工具安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，可以使用以下命令验证安装：

```bash
rustc --version
cargo --version
```

### 更新 Rust

```bash
rustup update
```

## 1.3 Rust 工具链

- **rustc**：Rust 编译器
- **cargo**：Rust 的包管理器和构建系统
- **rustup**：Rust 工具链管理器
- **rustfmt**：代码格式化工具
- **clippy**：代码检查工具

## 1.4 Hello, World!

创建你的第一个 Rust 程序：

```rust
fn main() {
    println!("Hello, World!");
}
```

保存为 `hello.rs`，然后编译并运行：

```bash
rustc hello.rs
./hello
```

## 1.5 Cargo 项目

Cargo 是 Rust 的构建系统和包管理器，使用 Cargo 创建新项目：

```bash
cargo new hello_cargo
cd hello_cargo
```

构建和运行项目：

```bash
cargo build    # 构建项目
cargo run      # 构建并运行项目
cargo check    # 检查代码但不生成可执行文件
cargo test     # 运行测试
```

## 1.6 变量和可变性

Rust 中的变量默认是不可变的：

```rust
let x = 5;      // 不可变变量
let mut y = 5;  // 可变变量
y = 6;          // 可以修改
```

常量：

```rust
const MAX_POINTS: u32 = 100_000;
```

变量遮蔽（shadowing）：

```rust
let x = 5;
let x = x + 1;  // 创建新变量，遮蔽旧变量
```

## 1.7 基本数据类型

### 标量类型

- **整数**：i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize
- **浮点数**：f32, f64
- **布尔值**：bool (true 或 false)
- **字符**：char (Unicode 字符)

```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'A';
```

### 复合类型

- **元组**：固定长度的不同类型值的集合

```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;  // 解构
let first = tup.0;     // 使用索引访问
```

- **数组**：固定长度的相同类型值的集合

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];    // 使用索引访问
```

## 1.8 函数

```rust
fn main() {
    println!("Hello from main!");
    another_function(5);
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    x + 1  // 返回值（无需 return 关键字）
}
```

## 1.9 注释

```rust
// 这是单行注释

/* 这是块注释，
   可以跨越多行 */

/// 文档注释，用于生成文档
/// # 示例
/// ```
/// let x = 5;
/// ```
```

## 本章示例

- [Hello World](./examples/hello_world/) - 最简单的 Rust 程序
- [Hello Cargo](./examples/hello_cargo/) - 使用 Cargo 创建的项目
- [变量示例](./examples/variables/) - 展示变量、数据类型和函数的使用

## 练习

1. 创建一个程序，声明不同类型的变量并打印它们的值
2. 编写一个函数，接受两个整数参数并返回它们的和
3. 创建一个程序，使用数组存储一周中的每一天，然后遍历并打印它们