// RefCell智能指针示例：展示Rust中RefCell<T>的基本用法和内部可变性模式

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 基本用法
    println!("=== RefCell的基本用法 ===\n");
    basic_usage();
    
    // 使用场景：内部可变性模式
    println!("\n=== 内部可变性模式 ===\n");
    interior_mutability();
    
    // 结合Rc和RefCell实现多所有权可变数据
    println!("\n=== 结合Rc和RefCell ===\n");
    rc_refcell_combination();
}

// 基本用法示例
fn basic_usage() {
    let data = RefCell::new(5);
    
    println!("初始值: {:?}", data.borrow());
    
    // 获取可变借用并修改值
    {
        let mut mut_ref = data.borrow_mut();
        *mut_ref += 10;
        println!("修改中: {:?} (通过可变借用)", mut_ref);
        
        // 注意：在可变借用存在期间，不能再获取其他借用
        // 下面的代码如果取消注释会导致运行时panic
        // let another_ref = data.borrow();
    }
    
    // 可变借用结束后，可以获取不可变借用
    let ref1 = data.borrow();
    let ref2 = data.borrow();
    
    println!("修改后: {:?} (通过不可变借用ref1)", ref1);
    println!("修改后: {:?} (通过不可变借用ref2)", ref2);
    
    // 演示try_borrow和try_borrow_mut
    println!("\n使用try_borrow和try_borrow_mut:");
    match data.try_borrow_mut() {
        Ok(mut r) => {
            *r += 5;
            println!("成功获取可变借用并修改: {:?}", r);
        }
        Err(_) => println!("获取可变借用失败"),
    }
    
    // 打印最终值
    println!("最终值: {:?}", data.borrow());
}

// 内部可变性模式示例
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
    
    fn send_message(&self, message: &str) {
        // 虽然self是不可变引用，但我们可以修改sent_messages
        self.sent_messages.borrow_mut().push(String::from(message));
    }
    
    fn sent_message_count(&self) -> usize {
        self.sent_messages.borrow().len()
    }
}

trait Messenger {
    fn send_message(&self, msg: &str);
}

impl Messenger for MockMessenger {
    fn send_message(&self, msg: &str) {
        self.send_message(msg);
    }
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send_message("错误：已超过配额！");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send_message("紧急警告：已使用超过90%的配额！");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send_message("警告：已使用超过75%的配额！");
        }
    }
}

fn interior_mutability() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
    println!("初始消息数: {}", mock_messenger.sent_message_count());
    
    limit_tracker.set_value(80);
    println!("使用80%后的消息数: {}", mock_messenger.sent_message_count());
    
    limit_tracker.set_value(95);
    println!("使用95%后的消息数: {}", mock_messenger.sent_message_count());
    
    limit_tracker.set_value(105);
    println!("使用105%后的消息数: {}", mock_messenger.sent_message_count());
    
    println!("发送的所有消息: {:?}", mock_messenger.sent_messages.borrow());
}

// 结合Rc和RefCell示例
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn rc_refcell_combination() {
    // 创建一个可以共享且可修改的整数
    let value = Rc::new(RefCell::new(5));
    
    // 创建引用这个值的链表
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    // 打印初始链表
    println!("初始值:");
    println!("a: {}", list_to_string(&a));
    println!("b: {}", list_to_string(&b));
    println!("c: {}", list_to_string(&c));
    
    // 修改共享值
    *value.borrow_mut() += 10;
    
    // 打印修改后的链表
    println!("\n修改共享值后:");
    println!("a: {}", list_to_string(&a));
    println!("b: {}", list_to_string(&b));
    println!("c: {}", list_to_string(&c));
}

fn list_to_string(list: &List) -> String {
    match list {
        Cons(value, next) => format!("{}(可变) -> {}", value.borrow(), list_to_string(next)),
        Nil => String::from("Nil"),
    }
}