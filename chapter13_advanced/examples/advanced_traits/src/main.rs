use std::ops::Add;

// 关联类型示例
trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 默认类型参数和运算符重载示例
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 为不同类型实现Add特征
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 完全限定语法示例
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("这里是机长说话...");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("起飞咯！");
    }
}

impl Human {
    fn fly(&self) {
        println!("*挥舞双臂*");
    }
}

// 关联函数的完全限定语法
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("小狗")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("幼犬")
    }
}

// 父特征示例（超特征）
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    // 关联类型示例
    let mut counter = Counter { count: 0 };
    println!("计数器: {:?}", counter.next());
    println!("计数器: {:?}", counter.next());
    
    // 运算符重载示例
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    
    println!("p3 = {:?}", p3);
    
    // 完全限定语法示例
    let person = Human;
    person.fly();           // 调用Human的方法
    Pilot::fly(&person);    // 调用Pilot特征的方法
    Wizard::fly(&person);   // 调用Wizard特征的方法
    
    // 关联函数的完全限定语法
    println!("普通方法: {}", Dog::baby_name());
    println!("特征方法: {}", <Dog as Animal>::baby_name());
    
    // 父特征示例
    let point = Point { x: 1, y: 2 };
    point.outline_print();
}