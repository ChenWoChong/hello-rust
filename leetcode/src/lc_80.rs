use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        return n as i32;
    }
    let (mut fast, mut slow) = (2, 2);
    while fast < n {
        if nums[slow - 2] != nums[fast] {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    let mut stack_size = 2;
    for i in 2..nums.len() {
        if nums[stack_size - 2] != nums[i] {
            nums[stack_size] = nums[i];
            stack_size += 1;
        }
    }
    stack_size.min(nums.len()) as _
}

pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
    let mut stack_size = 1;
    for i in 1..nums.len() {
        if nums[stack_size - 1] != nums[i] {
            nums[stack_size] = nums[i];
            stack_size += 1;
        }
    }
    stack_size.min(nums.len()) as _
}
