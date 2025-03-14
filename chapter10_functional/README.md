# 第十章：函数式特性

本章节将介绍 Rust 中的函数式编程特性，主要包括闭包和迭代器。

## 10.1 闭包

闭包是可以保存在变量中或作为参数传递给其他函数的匿名函数。与函数不同，闭包可以捕获其定义环境中的值。

### 闭包的基本语法

```rust
let add_one = |x| x + 1;
let add_two = |x: i32| -> i32 { x + 2 };

println!("add_one: {}", add_one(1));
println!("add_two: {}", add_two(1));
```

### 捕获环境中的值

闭包可以通过三种方式捕获其环境：

1. 借用不可变引用（`&T`）
2. 借用可变引用（`&mut T`）
3. 获取所有权（`T`）

```rust
let x = 4;

// 通过不可变引用捕获 x
let equal_to_x = |z| z == x;
assert!(equal_to_x(4));

// 通过可变引用捕获
let mut y = 5;
let mut add_to_y = |z| { y += z; y };
assert_eq!(10, add_to_y(5));

// 通过 move 关键字强制获取所有权
let x = vec![1, 2, 3];
let equal_to_x = move |z| z == x;
// 此处 x 已被移动到闭包中，不能再使用
```

### 函数和闭包作为参数

```rust
// 接受一个闭包作为参数的函数
fn apply_function<F>(f: F, x: i32) -> i32
    where F: Fn(i32) -> i32
{
    f(x)
}

let add_one = |x| x + 1;
let result = apply_function(add_one, 5);
assert_eq!(6, result);
```

### 闭包特质

Rust 提供了三种闭包特质：

1. `Fn`：通过不可变引用（`&T`）捕获
2. `FnMut`：通过可变引用（`&mut T`）捕获
3. `FnOnce`：获取所有权（`T`）捕获，只能被调用一次

```rust
fn call_once<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f());
}

fn call_mut<F>(mut f: F)
    where F: FnMut()
{
    f();
    f();
}

fn call_fn<F>(f: F)
    where F: Fn()
{
    f();
    f();
}
```

## 10.2 迭代器

迭代器是 Rust 中处理元素序列的方式。所有迭代器都实现了 `Iterator` 特质。

### 迭代器的基本用法

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();

assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
assert_eq!(Some(&3), iter.next());
assert_eq!(None, iter.next());
```

### 不同类型的迭代器

```rust
// iter() - 返回不可变引用的迭代器
let v1 = vec![1, 2, 3];
let iter1 = v1.iter();

// iter_mut() - 返回可变引用的迭代器
let mut v2 = vec![1, 2, 3];
let iter2 = v2.iter_mut();

// into_iter() - 返回获取所有权的迭代器
let v3 = vec![1, 2, 3];
let iter3 = v3.into_iter();
// 此处 v3 已被移动，不能再使用
```

### 常用的迭代器适配器

```rust
let v = vec![1, 2, 3, 4, 5];

// map - 对每个元素应用一个函数
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
assert_eq!(vec![2, 4, 6, 8, 10], doubled);

// filter - 只保留满足条件的元素
let even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
assert_eq!(vec![&2, &4], even);

// enumerate - 添加索引
let indexed: Vec<(usize, &i32)> = v.iter().enumerate().collect();
assert_eq!(vec![(0, &1), (1, &2), (2, &3), (3, &4), (4, &5)], indexed);

// zip - 将两个迭代器合并
let v2 = vec![10, 20, 30];
let zipped: Vec<(&i32, &i32)> = v.iter().zip(v2.iter()).collect();
assert_eq!(vec![(&1, &10), (&2, &20), (&3, &30)], zipped);
```

### 消费者适配器

```rust
let v = vec![1, 2, 3, 4, 5];

// sum - 计算总和
let sum: i32 = v.iter().sum();
assert_eq!(15, sum);

// fold - 折叠操作
let sum_plus_10 = v.iter().fold(10, |acc, x| acc + x);
assert_eq!(25, sum_plus_10);

// any - 检查是否有任何元素满足条件
let has_even = v.iter().any(|x| x % 2 == 0);
assert!(has_even);

// all - 检查是否所有元素都满足条件
let all_positive = v.iter().all(|x| *x > 0);
assert!(all_positive);
```

### 创建自定义迭代器

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 使用自定义迭代器
let counter = Counter::new(3);
let values: Vec<u32> = counter.collect();
assert_eq!(vec![1, 2, 3], values);
```

## 10.3 性能考虑

Rust 的迭代器是零成本抽象，这意味着使用迭代器通常不会引入运行时开销。编译器会将迭代器代码优化为与手写循环相当的机器代码。

闭包和迭代器的组合使用可以使代码更加简洁、可读，同时保持高性能。