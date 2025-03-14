// 单元测试示例：展示Rust中单元测试的基本用法

// 被测试的函数
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("你好，{}！", name)
}

pub fn panic_if_greater_than_100(value: i32) {
    if value > 100 {
        panic!("值必须小于等于 100，但得到了 {}", value);
    }
}

// 内部函数，但在测试中仍然可以被测试
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 测试模块
#[cfg(test)]
mod tests {
    // 导入上层模块的所有内容
    use super::*;

    // 基本的测试函数
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // 使用不同的断言宏
    #[test]
    fn greeting_contains_name() {
        let result = greeting("小明");
        assert!(result.contains("小明"), "问候语中没有包含名字");
    }

    // 测试应该发生panic的情况
    #[test]
    #[should_panic]
    fn greater_than_100() {
        panic_if_greater_than_100(200);
    }

    // 测试带有特定panic信息的情况
    #[test]
    #[should_panic(expected = "值必须小于等于 100")]
    fn greater_than_100_with_message() {
        panic_if_greater_than_100(200);
    }

    // 使用Result<T, E>的测试
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 不等于 4"))
        }
    }

    // 测试内部函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    // 被忽略的测试
    #[test]
    #[ignore]
    fn expensive_test() {
        // 假设这是一个耗时较长的测试
        let result = (0..1000000).fold(0, |acc, x| acc + x);
        assert!(result > 0);
    }
}