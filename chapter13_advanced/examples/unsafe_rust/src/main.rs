fn main() {
    // 裸指针示例
    let mut num = 5;
    
    // 创建指向数据的裸指针
    // 注意：创建裸指针是安全的，解引用才需要unsafe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("r1 的地址: {:?}", r1);
    println!("r2 的地址: {:?}", r2);
    
    // 在 unsafe 块中解引用裸指针
    unsafe {
        println!("r1 指向的值: {}", *r1);
        *r2 = 10;
        println!("修改后的值: {}", *r1);
    }
    
    // 不安全函数示例
    unsafe {
        dangerous();
    }
    
    // 使用外部代码的例子
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    unsafe {
        println!("C语言的abs(-3)的绝对值是: {}", abs(-3));
    }
    
    // 可变静态变量示例
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
    
    // 不安全特征示例
    unsafe {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..]; 
        let (a, b) = r.split_at_mut(3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}

// 不安全函数
unsafe fn dangerous() {
    println!("这是一个不安全函数");
}

// 可变静态变量
static mut COUNTER: u32 = 0;

// 安全抽象：使用不安全代码实现安全接口
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}