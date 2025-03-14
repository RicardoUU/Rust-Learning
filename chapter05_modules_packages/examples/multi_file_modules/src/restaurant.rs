// restaurant.rs - 餐厅模块

// 声明前台模块，但实现在另一个文件中
pub mod front_of_house;

// 使用从front_of_house模块中导入的函数
pub fn eat_at_restaurant() {
    println!("欢迎光临我们的餐厅！");
    
    // 使用模块中的函数
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
}