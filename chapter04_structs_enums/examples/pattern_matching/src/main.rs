// 模式匹配示例

// 定义一个硬币枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 定义一个美国州的枚举
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ... 其他州省略
}

fn main() {
    // 基本的 match 表达式
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("硬币的价值: {} 分", value);
    
    // 绑定值的模式
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("硬币的价值: {} 分", value);
    
    // 匹配 Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("\nOption 匹配示例:");
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
    
    // 通配符模式
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 处理所有其他值
    }
    
    // 忽略剩余值
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 不做任何事
    }
    
    // if let 简洁控制流
    let some_value = Some(3);
    
    // 使用 match
    match some_value {
        Some(3) => println!("\n使用 match: 值是三!"),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(3) = some_value {
        println!("使用 if let: 值是三!");
    }
    
    // 带 else 的 if let
    let mut count = 0;
    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alaska) {
        println!("\n来自{:?}州的25美分!", state);
    } else {
        count += 1;
    }
    
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    // 解构结构体的字段
    let Point { x, y } = p;
    println!("\n解构点坐标: x = {}, y = {}", x, y);
    
    // 在 match 中解构结构体
    match p {
        Point { x: 0, y } => println!("在y轴上，y = {}", y),
        Point { x, y: 0 } => println!("在x轴上，x = {}", x),
        Point { x, y } => println!("在其他位置: ({}, {})", x, y),
    }
}

// 计算硬币价值的函数
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运的便士!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自{:?}州的25美分硬币!", state);
            25
        },
    }
}

// Option 的加一函数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 用于通配符示例的函数
fn add_fancy_hat() {
    println!("\n添加了一顶fancy帽子!");
}

fn remove_fancy_hat() {
    println!("\n移除了一顶fancy帽子!");
}

fn reroll() {
    println!("\n重新掷骰子!");
}