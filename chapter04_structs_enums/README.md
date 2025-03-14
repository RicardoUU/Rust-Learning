# 第四章：结构化数据

本章节将介绍 Rust 中的结构化数据类型：结构体和枚举，以及它们的实现方法和模式匹配。

## 4.1 定义和实例化结构体

结构体是一种自定义数据类型，允许你命名和包装多个相关的值：

```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 访问结构体字段
    println!("用户名: {}", user1.username);
    
    // 可变结构体实例
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改字段值
    user2.email = String::from("newemail@example.com");
}
```

### 简化结构体创建

当变量名与字段名相同时，可以使用字段初始化简写语法：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email
        username, // 等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}
```

### 结构体更新语法

从其他实例创建新实例时，可以使用结构体更新语法：

```rust
let user3 = User {
    email: String::from("third@example.com"),
    ..user1 // 其余字段从 user1 获取
};
```

### 元组结构体

元组结构体有名称但字段没有名称：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 访问元组结构体字段
    println!("黑色的RGB值: ({}, {}, {})", black.0, black.1, black.2);
}
```

### 类单元结构体

没有任何字段的结构体：

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    // 可用于实现特定 trait 但不需要存储数据的情况
}
```

## 4.2 结构体方法

方法是定义在结构体上下文中的函数：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 带参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 关联函数（不接收 self 参数）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 调用方法
    println!("矩形面积: {}", rect1.area());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    // 调用带参数的方法
    println!("rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    
    // 调用关联函数
    let square = Rectangle::square(20);
    println!("正方形面积: {}", square.area());
}
```

## 4.3 枚举

枚举允许你定义一个类型，它可以是几个不同变体之一：

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    // 处理不同类型的 IP 地址
}
```

### 带数据的枚举

枚举的变体可以包含数据：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

### 复杂数据的枚举

枚举变体可以包含任意类型的数据，包括结构体：

```rust
enum Message {
    Quit,                       // 不包含数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含单个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32 值
}

impl Message {
    fn call(&self) {
        // 方法体
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### Option 枚举

Rust 标准库中的 Option 枚举表示一个值可能存在或不存在：

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // 使用 match 处理 Option
    match some_number {
        Some(n) => println!("数字是: {}", n),
        None => println!("没有数字"),
    }
}
```

## 4.4 模式匹配

### match 控制流

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式

```rust
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自{:?}州的25美分硬币!", state);
            25
        },
    }
}
```

### 匹配 Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### 通配符模式

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 处理所有其他值
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

### if let 简洁控制流

当只关心一种匹配模式时，可以使用 `if let` 简化 `match`：

```rust
fn main() {
    let some_value = Some(3);
    
    // 使用 match
    match some_value {
        Some(3) => println!("是三!"),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(3) = some_value {
        println!("是三!");
    }
}
```

## 本章示例

- [结构体](./examples/structs/) - 展示结构体的定义和使用
- [方法](./examples/methods/) - 展示结构体方法的实现
- [枚举](./examples/enums/) - 展示枚举的定义和使用
- [模式匹配](./examples/pattern_matching/) - 展示模式匹配的各种用法

## 练习

1. 创建一个 `Rectangle` 结构体，实现计算面积、周长和是否能容纳另一个矩形的方法
2. 创建一个表示文章的结构体，包含标题、作者、内容和发布状态，并实现相应的方法
3. 创建一个表示命令的枚举，包含不同类型的命令，并使用模式匹配处理这些命令