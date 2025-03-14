// 基本模块示例

// 在文件中定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding customer to waitlist...");
        }
    }
    
    mod serving {
        fn take_order() {
            println!("Taking order...");
        }
        
        // 演示如何使用pub(in)限制可见性
        pub(in crate::front_of_house) fn serve_order() {
            println!("Serving order...");
        }
    }
}

// 使用绝对路径访问模块中的函数
fn eat_at_restaurant_absolute() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
}

// 使用相对路径访问模块中的函数
fn eat_at_restaurant_relative() {
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    println!("模块示例");
    
    eat_at_restaurant_absolute();
    eat_at_restaurant_relative();
}