// 结构体方法示例

// 定义一个矩形结构体
struct Rectangle {
    width: u32,
    height: u32,
}

// 为结构体实现方法
impl Rectangle {
    // 计算面积的方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 判断是否能容纳另一个矩形的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 创建正方形的关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 创建矩形实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 调用方法计算面积
    println!("矩形的面积是: {} 平方像素", rect1.area());
    
    // 创建另一个矩形
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    // 调用方法判断是否能容纳其他矩形
    println!("rect1 能容纳 rect2 吗? {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3 吗? {}", rect1.can_hold(&rect3));
    
    // 使用关联函数创建一个正方形
    let square = Rectangle::square(25);
    println!("\n正方形的宽度: {}", square.width);
    println!("正方形的高度: {}", square.height);
    println!("正方形的面积: {}", square.area());
    
    // 多个 impl 块示例
    let rect4 = Rectangle {
        width: 35,
        height: 55,
    };
    
    println!("\nrect4 的周长是: {}", rect4.perimeter());
}

// 可以有多个 impl 块
impl Rectangle {
    // 计算周长的方法
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}