// 集成测试示例：展示Rust中集成测试的基本用法

// 被测试的函数
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn multiply_by_three(a: i32) -> i32 {
    a * 3
}

pub fn complex_operation(a: i32) -> i32 {
    add_two(multiply_by_three(a))
}

// 这个模块中的测试是单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_test() {
        assert_eq!(5, add_two(3));
        assert_eq!(9, multiply_by_three(3));
        assert_eq!(11, complex_operation(3));
    }
}