// 错误传播示例：展示Rust中的错误传播机制

use std::fs::File;
use std::io::{self, Read};

// 方法1：使用match手动传播错误
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  // 错误传播
    };
    
    let mut username = String::new();
    
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),  // 错误传播
    }
}

// 方法2：使用?运算符传播错误
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;  // ?运算符自动传播错误
    let mut username = String::new();
    file.read_to_string(&mut username)?;  // ?运算符自动传播错误
    Ok(username)
}

// 方法3：链式调用与?运算符
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 方法4：使用标准库函数
fn read_username_from_file_v4() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

// 自定义错误类型
#[derive(Debug)]
enum CustomError {
    IoError(io::Error),
    FormatError(String),
}

// 实现从io::Error到CustomError的转换
impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::IoError(error)
    }
}

// 使用自定义错误类型和?运算符
fn read_and_validate_username() -> Result<String, CustomError> {
    let username = std::fs::read_to_string("username.txt")?;  // io::Error自动转换为CustomError
    
    if username.trim().is_empty() {
        return Err(CustomError::FormatError("用户名不能为空".to_string()));
    }
    
    if username.len() < 3 {
        return Err(CustomError::FormatError("用户名太短".to_string()));
    }
    
    Ok(username.trim().to_string())
}

// 在main函数中使用?运算符
fn run() -> Result<(), CustomError> {
    let username = read_and_validate_username()?;
    println!("有效的用户名: {}", username);
    Ok(())
}

fn main() {
    println!("错误传播示例");
    
    // 测试不同版本的函数
    println!("\n1. 使用match手动传播错误");
    match read_username_from_file_v1() {
        Ok(name) => println!("读取到的用户名: {}", name),
        Err(e) => println!("错误: {:?}", e),
    }
    
    println!("\n2. 使用?运算符传播错误");
    match read_username_from_file_v2() {
        Ok(name) => println!("读取到的用户名: {}", name),
        Err(e) => println!("错误: {:?}", e),
    }
    
    println!("\n3. 链式调用与?运算符");
    match read_username_from_file_v3() {
        Ok(name) => println!("读取到的用户名: {}", name),
        Err(e) => println!("错误: {:?}", e),
    }
    
    println!("\n4. 使用标准库函数");
    match read_username_from_file_v4() {
        Ok(name) => println!("读取到的用户名: {}", name),
        Err(e) => println!(