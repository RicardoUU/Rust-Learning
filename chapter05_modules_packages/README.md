# 第五章：模块与包

本章节将介绍 Rust 的模块系统和包管理，帮助你组织和管理代码。

## 5.1 模块系统概述

Rust 的模块系统允许你将代码组织成独立的、可重用的单元，并控制项（函数、结构体等）的可见性。主要概念包括：

- **包（Package）**：Cargo 功能，用于构建、测试和共享 crate
- **Crate**：一个树形模块结构，可以产生库或可执行文件
- **模块（Module）**：控制代码组织、作用域和私有性
- **路径（Path）**：命名项（如结构体、函数或模块）的方式

## 5.2 定义模块

使用 `mod` 关键字定义模块：

```rust
// 在文件中定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    
    mod serving {
        fn take_order() {}
    }
}
```

## 5.3 模块树与路径

### 绝对路径与相对路径

```rust
// 绝对路径（从 crate 根开始）
use crate::front_of_house::hosting::add_to_waitlist;

// 相对路径（从当前模块开始）
use front_of_house::hosting::add_to_waitlist;
```

### super 关键字

使用 `super` 访问父模块：

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 调用父模块的函数
    }
    
    fn cook_order() {}
}
```

## 5.4 控制可见性

使用 `pub` 关键字控制项的可见性：

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // 公开字段
        seasonal_fruit: String, // 私有字段
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer { // 枚举成员默认公开
        Soup,
        Salad,
    }
}
```

## 5.5 use 关键字

使用 `use` 关键字将名称引入作用域：

```rust
// 基本用法
use crate::front_of_house::hosting;

// 使用 as 关键字重命名
use std::io::Result as IoResult;

// 重导出名称
pub use crate::front_of_house::hosting;

// 嵌套路径
use std::{cmp::Ordering, io};

// self 在路径中
use std::io::{self, Write};

// 通配符
use std::collections::*;
```

## 5.6 将模块拆分为多个文件

```rust
// src/lib.rs
mod front_of_house; // 声明模块，但实现在另一个文件中

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

```rust
// src/front_of_house.rs
pub mod hosting; // 声明子模块，但实现在另一个文件中
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

## 5.7 包和 Crate

### 创建库 Crate

```bash
cargo new my_library --lib
```

### Cargo.toml 配置

```toml
[package]
name = "my_package"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

### 工作空间（Workspace）

```toml
# Cargo.toml
[workspace]
members = [
    "adder",
    "add_one",
]
```

## 5.8 发布 Crate 到 crates.io

```bash
# 登录
cargo login your_api_key

# 发布
cargo publish
```

## 练习

1. 创建一个多模块的库，实现不同功能
2. 尝试使用工作空间管理多个相关的包
3. 探索标准库的模块结构