# 第十一章：智能指针

本章节将介绍 Rust 中的智能指针，这是一种数据结构，不仅像指针一样包含内存地址，还拥有额外的元数据和功能。

## 11.1 Box<T>

Box<T> 是最简单的智能指针，它允许你将值存储在堆上而不是栈上。

### 基本用法

```rust
let b = Box::new(5);
println!("b = {}", b); // 自动解引用
```

### 使用场景

1. **存储大型数据**：当你有一个大型数据结构，但不想在栈上复制它时。

2. **递归类型**：当你需要创建递归数据结构（如链表或树）时。

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

3. **特质对象**：当你需要存储实现了特定特质的类型，但不知道具体类型时。

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: {}", self.label);
    }
}

fn main() {
    let button = Box::new(Button {
        width: 50,
        height: 20,
        label: String::from("确定"),
    });
    
    let drawable: Box<dyn Draw> = button;
    drawable.draw();
}
```

## 11.2 Deref 特质

Deref 特质允许自定义解引用运算符 `*` 的行为。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用 MyBox
}
```

### 解引用强制转换

Rust 提供了解引用强制转换（deref coercion）功能，当一个类型实现了 Deref 特质时，Rust 会自动将该类型的引用转换为其 Deref 目标类型的引用。

```rust
fn hello(name: &str) {
    println!("你好，{}！", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 解引用强制转换：&MyBox<String> -> &String -> &str
}
```

## 11.3 Drop 特质

Drop 特质允许你自定义当值离开作用域时发生的行为。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("丢弃 CustomSmartPointer，数据：`{}`！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的数据"),
    };
    println!("CustomSmartPointer 已创建。");
    // c.drop(); // 错误：不能显式调用 drop
    drop(c); // 使用 std::mem::drop 提前丢弃值
    println!("CustomSmartPointer 在 main 结束前丢弃。");
}
```

## 11.4 Rc<T>

Rc<T>（引用计数）允许多个所有者共享同一数据的所有权。

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 创建后的引用计数 = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("b 创建后的引用计数 = {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c 创建后的引用计数 = {}", Rc::strong_count(&a));
    }
    
    println!("c 离开作用域后的引用计数 = {}", Rc::strong_count(&a));
}
```

## 11.5 RefCell<T>

RefCell<T> 提供内部可变性，允许你在拥有不可变引用时修改数据。

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // 获取可变借用
    let mut mut_ref = data.borrow_mut();
    *mut_ref += 1;
    
    // 尝试同时获取不可变借用会导致运行时错误（panic）
    // let ref1 = data.borrow(); // 这会导致 panic
    
    // 丢弃可变借用后可以获取不可变借用
    drop(mut_ref);
    let ref1 = data.borrow();
    let ref2 = data.borrow();
    
    println!("data = {}", *ref1);
}
```

## 11.6 结合 Rc<T> 和 RefCell<T>

结合 Rc<T> 和 RefCell<T> 可以创建具有多个所有者且可变的数据。

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    // 通过 a 修改值
    *a.borrow_mut() += 10;
    
    // 通过 b 修改值
    *b.borrow_mut() *= 2;
    
    println!("value = {}", *value.borrow());
}
```

## 11.7 循环引用和弱引用

使用 Rc<T> 可能导致循环引用，这会造成内存泄漏。Weak<T> 提供了一种不增加强引用计数的方式来引用数据。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf 强引用计数 = {}, 弱引用计数 = {}",
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        // 设置 leaf 的父节点为 branch，但使用弱引用
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("branch 强引用计数 = {}, 弱引用计数 = {}",
                 Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf 强引用计数 = {}, 弱引用计数 = {}",
                 Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    
    // branch 已离开作用域，应该被清理
    println!("branch 离开作用域后");
    
    // 尝试访问 leaf 的父节点
    println!("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf 强引用计数 = {}, 弱引用计数 = {}",
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
```