fn monotonic_stack(nums: &[i32], compare: fn(x: i32, y: i32) -> bool) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut result = vec![-1; nums.len()]; // 初始化结果数组，所有元素为-1

    for (i, &num) in nums.iter().enumerate() {
        // 当栈不为空且当前元素大于栈顶元素时
        while let Some(&top_index) = stack.last() {
            if compare(nums[top_index], num) {
                // 更新结果数组
                result[top_index] = num;
                // 弹出栈顶元素
                stack.pop();
            } else {
                // 当栈顶元素不小于当前元素时，停止循环
                break;
            }
        }
        // 将当前索引入栈
        stack.push(i);
    }

    result
}

fn compare_bigger(x: i32, y: i32) -> bool {
    x < y
}

fn compare_smaller(x: i32, y: i32) -> bool {
    x > y
}

fn main() {
    let nums = vec![2, 1, 2, 4, 3];
    let next_greater = monotonic_stack(&nums, compare_bigger);
    println!("Next greater elements: {:?}", next_greater);
    assert_eq!(next_greater, vec![4, 2, 4, -1, -1]);
    let next_smaller = monotonic_stack(&nums, compare_smaller);
    println!("Next smaller elements: {:?}", next_smaller);
    assert_eq!(next_smaller, vec![1, -1, -1, 3, -1]);
}
