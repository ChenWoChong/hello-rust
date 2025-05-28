impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![] as Vec<i32>;
        }

        let mut res = std::collections::BTreeSet::<i32>::new();
        if shorter == longer {
            res.insert(shorter * k);
            return res.into_iter().collect::<Vec<i32>>();
        }

        for i in 0..=k {
            res.insert(shorter * (k - i) + longer * i);
        }
        res.into_iter().collect::<Vec<i32>>()
    }
}
