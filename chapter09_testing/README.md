# 第九章：测试

本章节将介绍 Rust 中的测试功能，这是确保代码正确性的重要工具。

## 9.1 如何编写测试

Rust 中的测试是带有 `#[test]` 属性标注的函数。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

### 测试函数的结构

一个典型的测试函数包含以下部分：
1. 设置所需的数据或状态
2. 运行被测试的代码
3. 断言结果是否符合预期

### 常用断言宏

```rust
// 断言两个值相等
assert_eq!(expected, actual);

// 断言两个值不相等
assert_ne!(expected, actual);

// 断言一个条件为真
assert!(condition);

// 带有自定义错误信息的断言
assert!(condition, "错误信息: {}", error_details);
```

### 测试可能的失败情况

```rust
#[test]
#[should_panic]
fn greater_than_100() {
    panic_if_greater_than_100(200);
}

// 更精确地检查 panic 信息
#[test]
#[should_panic(expected = "值必须小于等于 100")]
fn greater_than_100_precise() {
    panic_if_greater_than_100(200);
}
```

### 使用 Result<T, E> 的测试

```rust
#[test]
fn it_works_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("2 + 2 不等于 4"))
    }
}
```

## 9.2 控制测试的运行方式

### 并行或串行运行测试

```bash
# 并行运行测试（默认）
cargo test

# 串行运行测试
cargo test -- --test-threads=1
```

### 显示函数输出

```bash
# 显示测试中的输出
cargo test -- --show-output
```

### 运行特定的测试

```bash
# 运行单个测试
cargo test test_name

# 运行包含特定字符串的测试
cargo test part_of_name
```

### 忽略测试

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 运行耗时较长的测试...
}
```

```bash
# 只运行被忽略的测试
cargo test -- --ignored
```

## 9.3 测试的组织结构

### 单元测试

单元测试与被测试的代码放在同一个文件中，通常位于 `tests` 模块中，并使用 `#[cfg(test)]` 属性标注。

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

### 集成测试

集成测试位于项目根目录的 `tests` 文件夹中，与 `src` 目录平级。

```
project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── integration_test.rs
    └── common/
        └── mod.rs
```

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn it_adds_two() {
    assert_eq!(4, my_crate::add_two(2));
}
```

### 测试私有函数

在 Rust 中，可以直接测试私有函数：

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## 9.4 测试驱动开发 (TDD)

测试驱动开发是一种软件开发方法，遵循以下步骤：

1. 编写一个会失败的测试，描述代码的期望行为
2. 编写最少量的代码使测试通过
3. 重构代码，保持测试通过
4. 重复上述步骤

这种方法有助于设计出更好的 API 和确保代码的正确性。