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
    
    println!("所有线程已完成");
    
    // 演示线程与闭包的交互 - 使用move关键字
    let v = vec![1, 2, 3];
    
    // 使用move将v的所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("在线程中访问向量: {:?}", v);
    });
    
    // 此处不能再使用v，因为所有权已转移
    // println!("在主线程中访问向量: {:?}", v); // 这行会导致编译错误
    
    handle.join().unwrap();
}