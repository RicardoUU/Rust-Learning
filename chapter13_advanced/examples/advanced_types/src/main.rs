fn main() {
    // 类型别名示例
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);
    
    // 复杂类型的别名
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let f: Thunk = Box::new(|| println!("你好"));
    f(); // 调用闭包
    
    // 在结果类型中使用类型别名
    type Result<T> = std::result::Result<T, std::io::Error>;
    
    // Never类型示例
    // 永不返回的函数
    fn forever() -> ! {
        loop {
            println!("永远循环...");
            // 这个函数永远不会返回
        }
    }
    
    // 在match表达式中使用never类型
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("匹配, n = {}", n),
        Some(n) => println!("Got a number: {}", n),
        None => panic!("不应该发生！"),  // panic! 的返回类型是 !
    }
    
    // 动态大小类型示例
    // str是一个DST（动态大小类型）
    // 我们通常使用&str而不是str
    let s1: &str = "Hello";
    let s2: &str = "世界";
    
    println!("{} {}", s1, s2);
    
    // 特征对象也是DST
    trait Draw {
        fn draw(&self);
    }
    
    struct Button {
        label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("绘制按钮: {}", self.label);
        }
    }
    
    struct SelectBox {
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("绘制选择框: {:?}", self.options);
        }
    }
    
    // 使用特征对象
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: String::from("确定") }),
        Box::new(SelectBox { options: vec![String::from("是"), String::from("否"), String::from("取消")] }),
    ];
    
    for component in components.iter() {
        component.draw();
    }
}