// 多文件模块示例

// 声明模块，但实现在另一个文件中
mod restaurant;

fn main() {
    println!("多文件模块示例");
    
    // 使用从restaurant模块中导入的函数
    restaurant::eat_at_restaurant();
}