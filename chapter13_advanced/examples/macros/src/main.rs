// 声明式宏示例
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
}

// 带参数的宏
macro_rules! vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 不同模式的宏
macro_rules! print_result {
    // 匹配单个表达式
    (expr $e:expr) => {
        println!("表达式: {} = {}", stringify!($e), $e);
    };
    // 匹配标识符和表达式
    (id $i:ident = $e:expr) => {
        let $i = $e;
        println!("{}的值是: {}", stringify!($i), $i);
    };
}

fn main() {
    // 使用简单宏
    say_hello!();
    
    // 使用带参数的宏
    let v = vec_macro![1, 2, 3, 4];
    println!("向量: {:?}", v);
    
    // 使用不同模式的宏
    print_result!(expr 5 + 6);
    print_result!(id value = 10 * 2);
    
    // 使用标准库中的宏
    let map = {
        let mut m = std::collections::HashMap::new();
        m.insert(1, "one");
        m.insert(2, "two");
        m
    };
    println!("映射: {:?}", map);
    
    // 使用debug_assert!宏（仅在调试模式下检查）
    let x = 5;
    debug_assert!(x == 5, "x应该等于5");
    
    // 使用format!宏创建格式化字符串
    let s = format!("Hello, {}!", "Rust");
    println!("{}", s);
    
    // 注意：过程宏需要在单独的crate中定义
    // 这里只是演示声明式宏
}