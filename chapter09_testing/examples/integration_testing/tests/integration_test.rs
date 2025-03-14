// 集成测试文件
// 注意：集成测试位于tests目录中，与src目录平级

// 导入被测试的crate
use integration_testing;

// 测试add_two函数
#[test]
fn test_add_two() {
    assert_eq!(5, integration_testing::add_two(3));
}

// 测试multiply_by_three函数
#[test]
fn test_multiply_by_three() {
    assert_eq!(12, integration_testing::multiply_by_three(4));
}

// 测试complex_operation函数
#[test]
fn test_complex_operation() {
    assert_eq!(14, integration_testing::complex_operation(4));
}