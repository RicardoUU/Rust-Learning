// 泛型示例：展示Rust中泛型的基本用法

// 泛型函数：查找最大值
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// 不同类型参数的泛型结构体
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// 为泛型结构体实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型的泛型结构体实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 泛型方法，带有不同的泛型参数
impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> MixedPoint<T, U> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("泛型示例");
    
    // 泛型函数示例
    println!("\n1. 泛型函数");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
    
    // 泛型结构体示例
    println!("\n2. 泛型结构体");
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: x={}, y={}", integer_point.x(), integer_point.y());
    println!("浮点数点: x={}, y={}", float_point.x(), float_point.y());
    
    // 特定类型实现的方法
    println!("\n3. 特定类型实现的方法");
    println!("到原点的距离: {}", float_point.distance_from_origin());
    
    // 不同类型参数的泛型结构体
    println!("\n4. 不同类型参数的泛型结构体");
    let mixed_point = MixedPoint { x: 5, y: 4.0 };
    println!("混合类型点: x={}, y={}", mixed_point.x, mixed_point.y);
    
    // 泛型方法示例
    println!("\n5. 泛型方法");
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: "World" };
    let p3 = p1.mixup(p2);
    println!("混合后的点: x={}, y={}", p3.x, p3.y);
    
    // 标准库中的泛型示例
    println!("\n6. 标准库中的泛型");
    let integer = Some(5);
    let string = Some("Hello");
    println!("Option<i32>: {:?}", integer);
    println!("Option<&str>: {:?}", string);
    
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了");
    println!("Result成功: {:?}", success);
    println!("Result失败: {:?}", failure);
}