struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut l, mut r) = (m-1 , n-1);
        for i in (0..m+n-1).rev() {
            println!("before: i:{}, l:{}[{}], r:{}[{}]", i, l,nums1[l as usize], r, nums2[r  as usize]);
            if r<0 || l>=0 && nums1[l as usize] >= nums2[r as usize] {
                nums1[i as usize] = nums1[l as usize];
                l -=1;
            } else {
                nums1[i as usize] = nums2[r as usize];
                r -=1;
            }
            println!("after: i:{}, l:{}[{}], r:{}[{}]", i, l,nums1[l as usize], r, nums2[r as usize]);
        }
    }
}