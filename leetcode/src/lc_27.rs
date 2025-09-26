use std::process::id;

struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx +=1;
            }
        }
        idx as i32
    }
}