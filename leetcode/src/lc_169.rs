struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut max = nums[0];
        let mut cnt = 0;
        let mut mp: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            if !mp.contains_key(&n) {
                mp.insert(n, 0);
            }

            let cur = mp.get(&n).unwrap() + 1;
            mp.insert(n, cur);
            if cur > cnt {
                max = n;
                cnt = cur;
            }
        }
        max
    }

    pub fn majority_element2(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut max = nums[0];
        let mut cnt = 0;
        let mut mp: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            let cur = mp.entry(*n).or_insert(0);
            *cur += 1;
            if *cur > cnt {
                cnt = *cur;
                max = *n;
            }
        }
        max
    }

    pub fn majority_element3(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}
