// 迭代器示例：展示Rust中迭代器的基本用法和特性

fn main() {
    // 迭代器的基本用法
    println!("=== 迭代器的基本用法 ===");
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    
    println!("第一个元素: {:?}", iter.next());
    println!("第二个元素: {:?}", iter.next());
    println!("第三个元素: {:?}", iter.next());
    println!("迭代结束: {:?}", iter.next());
    
    // 不同类型的迭代器
    println!("\n=== 不同类型的迭代器 ===");
    
    // iter() - 返回不可变引用的迭代器
    let v1 = vec![1, 2, 3];
    let mut iter1 = v1.iter();
    println!("iter()第一个元素: {:?}", iter1.next());
    println!("原始向量仍然可用: {:?}", v1);
    
    // iter_mut() - 返回可变引用的迭代器
    let mut v2 = vec![1, 2, 3];
    {
        let mut iter2 = v2.iter_mut();
        if let Some(first) = iter2.next() {
            *first += 10;
            println!("修改第一个元素为: {}", first);
        }
    }
    println!("修改后的向量: {:?}", v2);
    
    // into_iter() - 返回获取所有权的迭代器
    let v3 = vec![1, 2, 3];
    let mut iter3 = v3.into_iter();
    println!("into_iter()第一个元素: {:?}", iter3.next());
    // 此处v3已被移动，不能再使用
    // println!("v3: {:?}", v3); // 这行会导致编译错误
    
    // 常用的迭代器适配器
    println!("\n=== 常用的迭代器适配器 ===");
    let v = vec![1, 2, 3, 4, 5];
    
    // map - 对每个元素应用一个函数
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map - 每个元素乘以2: {:?}", doubled);
    
    // filter - 只保留满足条件的元素
    let even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter - 只保留偶数: {:?}", even);
    
    // enumerate - 添加索引
    let indexed: Vec<(usize, &i32)> = v.iter().enumerate().collect();
    println!("enumerate - 添加索引: {:?}", indexed);
    
    // zip - 将两个迭代器合并
    let v2 = vec![10, 20, 30, 40, 50];
    let zipped: Vec<(&i32, &i32)> = v.iter().zip(v2.iter()).collect();
    println!("zip - 合并两个迭代器: {:?}", zipped);
    
    // 消费者适配器
    println!("\n=== 消费者适配器 ===");
    
    // sum - 计算总和
    let sum: i32 = v.iter().sum();
    println!("sum - 计算总和: {}", sum);
    
    // fold - 折叠操作
    let sum_plus_10 = v.iter().fold(10, |acc, x| acc + x);
    println!("fold - 从10开始累加: {}", sum_plus_10);
    
    // any - 检查是否有任何元素满足条件
    let has_even = v.iter().any(|x| x % 2 == 0);
    println!("any - 是否有偶数: {}", has_even);
    
    // all - 检查是否所有元素都满足条件
    let all_positive = v.iter().all(|x| *x > 0);
    println!("all - 是否全部为正数: {}", all_positive);
    
    // 自定义迭代器
    println!("\n=== 自定义迭代器 ===");
    let counter = Counter::new(3);
    let values: Vec<u32> = counter.collect();
    println!("自定义计数器迭代器: {:?}", values);
    
    // 链式调用
    println!("\n=== 链式调用 ===");
    let sum_of_squares: u32 = Counter::new(5)
        .map(|x| x * x)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("计数器的平方中能被3整除的数之和: {}", sum_of_squares);
}

// 自定义迭代器
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 为自定义迭代器实现其他迭代器特质方法
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        (self.max - self.count) as usize
    }
}