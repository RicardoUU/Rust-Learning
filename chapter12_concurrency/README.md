# 第12章：并发编程

并发编程是现代软件开发中的重要组成部分，尤其是在多核处理器普及的今天。Rust 提供了强大的并发编程工具，同时通过其所有权和类型系统确保线程安全。

## 本章内容

- 线程基础：创建线程、等待线程完成
- 消息传递：使用通道在线程间传递数据
- 共享状态：使用互斥锁和读写锁安全地共享数据
- 并发抽象：探索 Rust 标准库和第三方库提供的并发原语

## 线程基础

在 Rust 中，可以使用 `std::thread` 模块创建线程：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("线程中: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程中的代码
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待线程完成
    handle.join().unwrap();
}
```

## 消息传递

Rust 采用通道（channel）进行线程间通信，这是一种无共享内存的消息传递机制：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个通道
    let (tx, rx) = mpsc::channel();

    // 创建线程，移动发送端到线程中
    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
        // 此处 val 已被移动，不能再使用
    });

    // 在主线程中接收消息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
```

## 共享状态

当需要在线程间共享数据时，可以使用互斥锁（Mutex）：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc 提供线程安全的引用计数
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap());
}
```

## 示例代码

本章包含以下示例：

1. [基本线程](./examples/threads/): 线程创建和管理
2. [消息传递](./examples/message_passing/): 使用通道在线程间通信
3. [共享状态](./examples/shared_state/): 使用互斥锁安全地共享数据

## 练习

1. 创建一个多线程程序，使用线程池处理任务
2. 实现一个简单的生产者-消费者模型
3. 使用并发技术优化之前章节中的某个示例

## 进一步学习

- [Rayon](https://github.com/rayon-rs/rayon): 数据并行库
- [Tokio](https://tokio.rs/): 异步运行时
- [crossbeam](https://github.com/crossbeam-rs/crossbeam): 并发工具集