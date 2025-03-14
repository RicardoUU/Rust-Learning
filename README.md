# Rust 学习工程

欢迎来到 Rust 学习工程！这个项目旨在帮助你系统地学习 Rust 编程语言，从基础概念到高级特性。

## 项目结构

本项目按章节组织，每个章节包含：
- 一个详细的 README.md 文件，解释该章节的核心概念
- 一个或多个实例代码，展示概念的实际应用
- 练习题（可选）帮助巩固所学知识

## 章节列表

1. [Rust 基础入门](./chapter01_basics/README.md) - 安装、变量、基本数据类型
2. [控制流](./chapter02_control_flow/README.md) - 条件语句、循环、模式匹配
3. [所有权系统](./chapter03_ownership/README.md) - Rust 的核心特性：所有权、借用、生命周期
4. [结构化数据](./chapter04_structs_enums/README.md) - 结构体、枚举、模式匹配
5. [模块与包](./chapter05_modules_packages/README.md) - 代码组织、可见性、Cargo 包管理
6. [常见集合](./chapter06_collections/README.md) - Vector、String、HashMap
7. [错误处理](./chapter07_error_handling/README.md) - Result、Option、错误传播
8. [泛型与特征](./chapter08_generics_traits/README.md) - 泛型、特征、生命周期
9. [测试](./chapter09_testing/README.md) - 单元测试、集成测试
10. [函数式特性](./chapter10_functional/README.md) - 闭包、迭代器
11. [智能指针](./chapter11_smart_pointers/README.md) - Box、Rc、RefCell
12. [并发编程](./chapter12_concurrency/README.md) - 线程、消息传递、共享状态
13. [Rust 高级特性](./chapter13_advanced/README.md) - 不安全 Rust、高级特征
14. [实战项目](./chapter14_project/README.md) - 综合应用所学知识

## 如何使用本项目

1. 按顺序学习每个章节
2. 阅读章节的 README.md 文件，理解概念
3. 研究并运行示例代码
4. 尝试修改示例代码，实验不同的用法
5. 完成章节练习（如果有）

## 运行代码

每个章节的示例代码都可以使用 Rust 的标准工具运行：

```bash
# 进入特定章节的示例目录
cd chapter01_basics/examples/hello_world

# 编译并运行代码
cargo run
```

## 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 程序设计语言（中文版）](https://kaisery.github.io/trpl-zh-cn/)
- [Rust by Example（中文版）](https://rustwiki.org/zh-CN/rust-by-example/)

祝你学习愉快！