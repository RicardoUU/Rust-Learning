fn main() {
    // 变量和可变性
    let x = 5;
    println!("不可变变量 x 的值: {}", x);
    
    let mut y = 5;
    println!("可变变量 y 的初始值: {}", y);
    y = 6;
    println!("可变变量 y 的新值: {}", y);
    
    // 变量遮蔽
    let z = 10;
    println!("变量 z 的初始值: {}", z);
    let z = z + 5;
    println!("变量遮蔽后 z 的新值: {}", z);
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS 的值: {}", MAX_POINTS);
    
    // 基本数据类型
    // 整数
    let integer: i32 = 42;
    println!("整数: {}", integer);
    
    // 浮点数
    let float: f64 = 3.14;
    println!("浮点数: {}", float);
    
    // 布尔值
    let boolean: bool = true;
    println!("布尔值: {}", boolean);
    
    // 字符
    let character: char = 'A';
    println!("字符: {}", character);
    
    // 复合类型
    // 元组
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (a, b, c) = tup; // 解构
    println!("元组解构: a = {}, b = {}, c = {}", a, b, c);
    println!("元组索引访问: tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
    
    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组第一个元素: {}", arr[0]);
    println!("数组第二个元素: {}", arr[1]);
    
    // 调用函数
    let result = another_function(5);
    println!("函数返回值: {}", result);
    
    // 数组遍历示例
    let days = ["星期一", "星期二", "星期三", "星期四", "星期五", "星期六", "星期日"];
    println!("一周的天数:");
    for day in days.iter() {
        println!("{}", day);
    }
}

// 函数定义
fn another_function(x: i32) -> i32 {
    println!("函数参数 x 的值: {}", x);
    x + 1 // 返回值（无需 return 关键字）
}