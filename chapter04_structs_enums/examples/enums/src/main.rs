// 枚举示例

// 定义一个简单的 IP 地址类型枚举
enum IpAddrKind {
    V4,
    V6,
}

// 定义一个带有数据的 IP 地址枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 定义一个包含不同类型数据的消息枚举
enum Message {
    Quit,                       // 不包含数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含单个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32 值
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        // 这里可以根据枚举变体类型执行不同的操作
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标: ({}, {})", x, y),
            Message::Write(text) => println!("文本消息: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为: RGB({}, {}, {})", r, g, b),
        }
    }
}

// 定义一个硬币枚举用于后面的模式匹配示例
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // 使用简单枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(four);
    route(six);
    
    // 使用带数据的枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("\nIP 地址示例:");
    print_ip_addr(&home);
    print_ip_addr(&loopback);
    
    // 使用复杂数据的枚举
    let quit = Message::Quit;
    let mv = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Hello"));
    let color = Message::ChangeColor(255, 0, 255);
    
    println!("\n消息示例:");
    quit.call();
    mv.call();
    write.call();
    color.call();
    
    // Option 枚举示例
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("\nOption 示例:");
    print_option(&some_number);
    print_option(&some_string);
    print_option(&absent_number);
    
    // 使用 match 处理枚举
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    
    println!("\n硬币价值示例:");
    println!("Penny 的价值: {} 分", value_in_cents(penny));
    println!("Nickel 的价值: {} 分", value_in_cents(nickel));
    println!("Dime 的价值: {} 分", value_in_cents(dime));
    println!("Quarter 的价值: {} 分", value_in_cents(quarter));
    
    // Option 的 match 处理
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("\nOption 匹配示例:");
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
    
    // if let 简洁控制流
    let some_value = Some(3);
    
    println!("\nif let 示例:");
    if let Some(3) = some_value {
        println!("值是三!");
    }
}

// 处理 IP 地址类型的函数
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由 IPv4 地址"),
        IpAddrKind::V6 => println!("路由 IPv6 地址"),
    }
}

// 打印 IP 地址的函数
fn print_ip_addr(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4 地址: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("IPv6 地址: {}", s),
    }
}

// 打印 Option 值的函数
fn print_option<T: std::fmt::Debug>(opt: &Option<T>) {
    match opt {
        Some(value) => println!("有值: {:?}", value),
        None => println!("没有值"),
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
        Coin::Quarter => 25,
    }
}

// Option 的加一函数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}