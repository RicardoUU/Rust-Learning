fn main() {
    println!("所有权示例");
    
    // 基本的所有权规则
    ownership_basics();
    
    // 移动语义
    move_semantics();
    
    // 克隆
    clone_example();
    
    // 栈上数据的复制
    copy_trait_example();
    
    // 函数与所有权
    functions_and_ownership();
}

// 基本的所有权规则示例
fn ownership_basics() {
    println!("\n--- 基本的所有权规则 ---");
    
    // 变量作用域
    {
        let s = String::from("hello"); // s 在此处有效
        println!("s: {}", s);
    } // 此作用域已结束，s 不再有效
    
    // 以下代码会导致编译错误，因为 s 已经不在作用域内
    // println!("s: {}", s);
    
    // 多个变量的作用域
    let x = 5;
    let y = x; // 复制 x 的值到 y
    
    println!("x = {}, y = {}", x, y); // 两个变量都可以使用
}

// 移动语义示例
fn move_semantics() {
    println!("\n--- 移动语义 ---");
    
    let s1 = String::from("hello");
    println!("创建 s1: {}", s1);
    
    let s2 = s1; // s1 的所有权移动到 s2
    println!("s2: {}", s2);
    
    // 以下代码会导致编译错误，因为 s1 的值已被移动
    // println!("s1: {}", s1);
    
    // 移动也会发生在函数调用中
    let s3 = String::from("world");
    println!("创建 s3: {}", s3);
    takes_ownership(s3);
    // 以下代码会导致编译错误，因为 s3 的值已被移动到函数中
    // println!("s3: {}", s3);
}

// 克隆示例
fn clone_example() {
    println!("\n--- 克隆 ---");
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深度复制 s1 的数据
    
    println!("s1: {}", s1); // 正确，s1 仍然有效
    println!("s2: {}", s2); // 正确，s2 是 s1 的独立副本
}

// Copy trait 示例
fn copy_trait_example() {
    println!("\n--- Copy trait ---");
    
    // 实现了 Copy trait 的类型
    let x = 5;
    let y = x; // x 的值被复制给 y
    
    println!("x: {}", x); // 正确，x 仍然有效
    println!("y: {}", y); // 正确
    
    // 其他实现了 Copy trait 的类型
    let a = true;
    let b = a;
    println!("a: {}, b: {}", a, b);
    
    let c = 3.14;
    let d = c;
    println!("c: {}, d: {}", c, d);
    
    let e = 'e';
    let f = e;
    println!("e: {}, f: {}", e, f);
    
    let g = (1, 2, 3);
    let h = g;
    println!("g: {:?}, h: {:?}", g, h);
}

// 函数与所有权示例
fn functions_and_ownership() {
    println!("\n--- 函数与所有权 ---");
    
    let s = String::from("hello"); // s 进入作用域
    
    takes_ownership(s); // s 的值移动到函数中
    // s 不再有效
    
    let x = 5; // x 进入作用域
    
    makes_copy(x); // x 的值被复制到函数中
    // x 仍然有效
    println!("x 在函数调用后仍然有效: {}", x);
    
    // 返回值与所有权
    let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
    println!("s1: {}", s1);
    
    let s2 = String::from("hello"); // s2 进入作用域
    println!("创建 s2: {}", s2);
    
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数中，函数返回值移给 s3
    println!("s3: {}", s3);
    // 此时 s2 已无效，但 s3 有效
}

// 接受所有权的函数
fn takes_ownership(some_string: String) {
    println!("接收了字符串: {}", some_string);
} // some_string 离开作用域并调用 `drop` 方法，内存被释放

// 接受复制的函数
fn makes_copy(some_integer: i32) {
    println!("接收了整数: {}", some_integer);
} // some_integer 离开作用域，不会有特殊操作

// 返回所有权的函数
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

// 接受并返回所有权的函数
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移出给调用的函数
}