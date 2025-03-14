// Rc智能指针示例：展示Rust中Rc<T>的基本用法和应用场景

use std::rc::Rc;

fn main() {
    // 基本用法
    println!("=== Rc的基本用法 ===");
    let a = Rc::new(5);
    println!("a = {}, 引用计数 = {}", a, Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("创建b后，a的引用计数 = {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("创建c后，a的引用计数 = {}", Rc::strong_count(&a));
    }
    
    println!("c离开作用域后，a的引用计数 = {}", Rc::strong_count(&a));
    
    // 使用场景：多个所有者共享数据
    println!("\n=== 使用Rc实现多所有权的链表 ===");
    let list_a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("list_a创建后的引用计数 = {}", Rc::strong_count(&list_a));
    
    let list_b = Cons(3, Rc::clone(&list_a));
    println!("list_b创建后，list_a的引用计数 = {}", Rc::strong_count(&list_a));
    
    let list_c = Cons(4, Rc::clone(&list_a));
    println!("list_c创建后，list_a的引用计数 = {}", Rc::strong_count(&list_a));
    
    // 打印链表
    println!("list_a: {}", list_to_string(&(*list_a)));
    println!("list_b: {}", list_to_string(&list_b));
    println!("list_c: {}", list_to_string(&list_c));
}

// 使用Rc实现的链表
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn list_to_string(list: &List) -> String {
    match list {
        Cons(value, next) => format!("{} -> {}", value, list_to_string(&*next)),
        Nil => String::from("Nil"),
    }
}