use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个通道
    let (tx, rx) = mpsc::channel();

    // 创建线程，移动发送端到线程中
    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
        // 此处 val 已被移动，不能再使用
    });

    // 在主线程中接收消息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
    
    // 多个消息的发送与接收
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("消息1"),
            String::from("消息2"),
            String::from("消息3"),
            String::from("消息4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 使用迭代器接收多个消息
    for received in rx {
        println!("收到: {}", received);
    }
    
    // 多个生产者，一个消费者
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("来自线程1: 消息1"),
            String::from("来自线程1: 消息2"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("来自线程2: 消息1"),
            String::from("来自线程2: 消息2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("收到: {}", received);
    }
}