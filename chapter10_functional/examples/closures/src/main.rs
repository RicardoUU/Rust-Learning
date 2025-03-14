// 闭包示例：展示Rust中闭包的基本用法和特性

fn main() {
    // 基本闭包语法
    println!("=== 基本闭包语法 ===");
    let add_one = |x| x + 1;
    let add_two = |x: i32| -> i32 { x + 2 };
    
    println!("add_one(1) = {}", add_one(1));
    println!("add_two(1) = {}", add_two(1));
    
    // 捕获环境中的值
    println!("\n=== 捕获环境中的值 ===");
    
    // 通过不可变引用捕获
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("equal_to_x(4) = {}", equal_to_x(4));
    println!("equal_to_x(5) = {}", equal_to_x(5));
    
    // 通过可变引用捕获
    let mut y = 5;
    {
        let mut add_to_y = |z| { y += z; y };
        println!("调用add_to_y(2) = {}", add_to_y(2));
        println!("调用add_to_y(3) = {}", add_to_y(3));
    }
    println!("最终y的值 = {}", y);
    
    // 通过move关键字强制获取所有权
    let v = vec![1, 2, 3];
    let contains = move |needle| v.contains(&needle);
    println!("contains(2) = {}", contains(2));
    println!("contains(5) = {}", contains(5));
    // 此处v已被移动到闭包中，不能再使用
    // println!("v = {:?}", v); // 这行会导致编译错误
    
    // 函数和闭包作为参数
    println!("\n=== 函数和闭包作为参数 ===");
    let add_one_closure = |x| x + 1;
    let result = apply_function(add_one_closure, 5);
    println!("apply_function(add_one, 5) = {}", result);
    
    // 闭包特质
    println!("\n=== 闭包特质 ===");
    
    // FnOnce - 获取所有权，只能被调用一次
    let text = String::from("你好");
    let consume_and_return = move || {
        let mut s = text;
        s.push_str(", 世界");
        s
    };
    
    let result = call_once(consume_and_return);
    println!("call_once结果: {}", result);
    
    // FnMut - 可变引用
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("计数器: {}", counter);
    };
    
    call_mut(&mut increment);
    
    // Fn - 不可变引用
    let value = 5;
    let print_value = || println!("值: {}", value);
    
    call_fn(&print_value);
    
    // 闭包作为返回值
    println!("\n=== 闭包作为返回值 ===");
    let multiplier = create_multiplier(3);
    println!("multiplier(4) = {}", multiplier(4));
    println!("multiplier(5) = {}", multiplier(5));
}

// 接受一个闭包作为参数的函数
fn apply_function<F>(f: F, x: i32) -> i32
    where F: Fn(i32) -> i32
{
    f(x)
}

// FnOnce - 获取所有权，只能被调用一次
fn call_once<F>(f: F) -> String
    where F: FnOnce() -> String
{
    f()
}

// FnMut - 可变引用
fn call_mut<F>(f: &mut F)
    where F: FnMut()
{
    f();
    f();
}

// Fn - 不可变引用
fn call_fn<F>(f: &F)
    where F: Fn()
{
    f();
    f();
}

// 返回一个闭包
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}