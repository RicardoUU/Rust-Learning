// Box智能指针示例：展示Rust中Box<T>的基本用法和应用场景

fn main() {
    // 基本用法 - 在堆上分配值
    println!("=== Box的基本用法 ===");
    let b = Box::new(5);
    println!("b = {}", b); // 自动解引用
    
    // 使用场景1：存储大型数据
    println!("\n=== 使用Box存储大型数据 ===");
    let large_data = Box::new([0; 1000]); // 1000个元素的数组存储在堆上
    println!("large_data的前5个元素: {:?}", &large_data[0..5]);
    
    // 使用场景2：递归类型
    println!("\n=== 使用Box实现递归类型 ===");
    let list = create_list();
    println!("链表: {}", list_to_string(&list));
    
    // 使用场景3：特质对象
    println!("\n=== 使用Box存储特质对象 ===");
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 50,
            height: 20,
            label: String::from("确定"),
        }),
        Box::new(SelectBox {
            width: 100,
            height: 30,
            options: vec![String::from("是"), String::from("否"), String::from("取消")],
        }),
    ];
    
    draw_screen(&components);
}

// 递归类型示例 - 链表
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn create_list() -> List {
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
}

fn list_to_string(list: &List) -> String {
    match list {
        Cons(value, next) => format!("{} -> {}", value, list_to_string(next)),
        Nil => String::from("Nil"),
    }
}

// 特质对象示例
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
        println!("绘制按钮: {} ({}x{})", self.label, self.width, self.height);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制选择框: {}x{} 选项: {:?}", self.width, self.height, self.options);
    }
}

fn draw_screen(components: &[Box<dyn Draw>]) {
    for component in components {
        component.draw();
    }
}