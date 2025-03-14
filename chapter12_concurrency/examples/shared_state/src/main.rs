use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 基本的互斥锁使用
    let m = Mutex::new(5);
    
    {
        // 获取锁并修改其中的值
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 锁在这里被释放
    
    println!("m = {:?}", m);
    
    // 在多线程间共享可变数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        // 克隆Arc，增加引用计数
        let counter = Arc::clone(&counter);
        
        // 创建新线程
        let handle = thread::spawn(move || {
            // 在线程中获取锁并修改值
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果: {}", *counter.lock().unwrap());
    
    // 演示死锁风险（注释掉以避免实际死锁）
    /*
    let mutex = Mutex::new(0);
    
    // 获取锁
    let mut data = mutex.lock().unwrap();
    *data += 1;
    
    // 在同一作用域内尝试再次获取锁 - 这会导致死锁
    // let mut data2 = mutex.lock().unwrap(); // 这行会导致死锁
    */
    
    // 读写锁示例 (RwLock)
    use std::sync::RwLock;
    
    let rw_lock = RwLock::new(5);
    
    // 多个读取者可以同时获取读锁
    {
        let r1 = rw_lock.read().unwrap();
        let r2 = rw_lock.read().unwrap();
        println!("读取者: {} {}", *r1, *r2);
    } // 读锁在这里被释放
    
    // 只有一个写入者可以获取写锁
    {
        let mut w = rw_lock.write().unwrap();
        *w += 1;
        println!("写入后: {}", *w);
    } // 写锁在这里被释放
}