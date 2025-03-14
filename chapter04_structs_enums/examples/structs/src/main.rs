// 结构体示例

// 定义一个用户结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 定义一个颜色的元组结构体
struct Color(i32, i32, i32);

// 定义一个没有任何字段的单元结构体
struct AlwaysEqual;

fn main() {
    // 创建一个用户实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 访问结构体字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    println!("活跃状态: {}", user1.active);
    println!("登录次数: {}", user1.sign_in_count);
    
    // 创建可变的结构体实例
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改字段值
    user2.email = String::from("newemail@example.com");
    println!("\n修改后的邮箱: {}", user2.email);
    
    // 使用函数创建结构体实例
    let user3 = build_user(
        String::from("third@example.com"),
        String::from("thirduser789")
    );
    println!("\n新用户邮箱: {}", user3.email);
    
    // 使用结构体更新语法
    let user4 = User {
        email: String::from("fourth@example.com"),
        ..user1 // 其余字段从 user1 获取
    };
    println!("\n使用更新语法创建的用户名: {}", user4.username);
    
    // 使用元组结构体
    let black = Color(0, 0, 0);
    println!("\n黑色的RGB值: ({}, {}, {})", black.0, black.1, black.2);
    
    // 使用单元结构体
    let _subject = AlwaysEqual;
    println!("\n创建了一个单元结构体实例");
}

// 创建用户的函数
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email
        username, // 等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}