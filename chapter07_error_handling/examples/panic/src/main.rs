// panic示例：展示Rust中的panic机制和不可恢复错误处理

fn main() {
    println!("Rust的panic机制示例");
    
    // 基本的panic示例
    basic_panic_demo();
    
    // 数组越界导致的panic
    // array_out_of_bounds();
    
    // 使用unwrap和expect
    unwrap_and_expect_demo();
    
    // 自定义panic条件
    custom_panic_conditions();
    
    println!("\n如果你看到这条消息，说明没有触发panic或者panic被注释掉了");
}

// 基本的panic示例
fn basic_panic_demo() {
    println!("\n1. 基本的panic示例");
    println!("注意：下面的代码被注释掉了，取消注释可以看到panic效果");
    
    // 直接调用panic宏
    // panic!("这是一个手动触发的panic!");
    
    println!("这行代码在panic被注释的情况下才会执行");
}

// 数组越界导致的panic
fn array_out_of_bounds() {
    println!("\n2. 数组越界导致的panic");
    let v = vec![1, 2, 3];
    
    // 以下代码会导致panic，因为尝试访问索引为99的元素，但数组中没有这个索引
    let value = v[99]; // 这会导致panic
    println!("这行代码永远不会执行: {}", value);
}

// 使用unwrap和expect
fn unwrap_and_expect_demo() {
    println!("\n3. unwrap和expect示例");
    
    // 使用unwrap处理Option
    let name: Option<&str> = None;
    println!("使用unwrap处理None会导致panic，下面的代码被注释掉了");
    // let unwrapped_name = name.unwrap(); // 这会导致panic
    
    // 使用expect处理Option
    println!("使用expect处理None会导致panic并显示自定义消息，下面的代码被注释掉了");
    // let expected_name = name.expect("名字不应该为None"); // 这会导致panic
    
    // 使用unwrap处理Result
    let result: Result<i32, &str> = Err("发生了错误");
    println!("使用unwrap处理Err会导致panic，下面的代码被注释掉了");
    // let unwrapped_result = result.unwrap(); // 这会导致panic
    
    // 使用expect处理Result
    println!("使用expect处理Err会导致panic并显示自定义消息，下面的代码被注释掉了");
    // let expected_result = result.expect("结果不应该是Err"); // 这会导致panic
    
    // 正确使用unwrap的情况
    let valid_option: Option<&str> = Some("有效值");
    let valid_unwrap = valid_option.unwrap();
    println!("正确使用unwrap: {}", valid_unwrap);
    
    let valid_result: Result<i32, &str> = Ok(42);
    let valid_result_unwrap = valid_result.unwrap();
    println!("正确使用Result的unwrap: {}", valid_result_unwrap);
}

// 自定义panic条件
fn custom_panic_conditions() {
    println!("\n4. 自定义panic条件");
    
    let age = 15;
    
    // 使用断言
    println!("使用assert!宏进行断言，如果条件为false则panic");
    assert!(age >= 0, "年龄不能为负数"); // 这不会panic
    
    // 使用debug断言（仅在debug模式下检查）
    println!("使用debug_assert!宏进行断言，仅在debug模式下检查");
    debug_assert!(age >= 0, "年龄不能为负数"); // 这不会panic
    
    // 条件性panic
    println!("条件性panic，下面的代码被注释掉了");
    // if age < 18 {
    //     panic!("未成年人不允许访问！当前年龄: {}", age);
    // }
    
    println!("自定义验证函数");
    fn validate_age(age: i32) -> Result<(), &'static str> {
        if age < 0 {
            return Err("年龄不能为负数");
        }
        if age > 150 {
            return Err("年龄不太可能超过150岁");
        }
        Ok(())
    }
    
    // 使用自定义验证函数
    match validate_age(age) {
        Ok(()) => println!("年龄验证通过: {}", age),
        Err(e) => {
            println!("年龄验证失败: {}", e);
            // 可以选择在这里panic
            // panic!("年龄验证失败: {}", e);
        }
    }
}