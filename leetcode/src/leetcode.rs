#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // map
        // for strs
        // sort chars
        // add to map
        // for strs to gen result
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::default();
        for str in strs {
            let mut str_char = str.chars().collect::<Vec<char>>();
            str_char.sort();
            let sort_str: String = str_char.into_iter().collect();
            if map.contains_key(&sort_str) {
                let mut key_map = map.get_mut(&sort_str).unwrap().to_owned();
                key_map.push(str);
                map.insert(sort_str, key_map);
            } else {
                let mut key_map: Vec<String> = Vec::new();
                key_map.push(str);
                map.insert(sort_str, key_map);
            }
        }

        let mut res: Vec<Vec<String>> = Vec::new();
        for (_, v) in map {
            res.push(v);
        }
        res
    }

    #[allow(dead_code)]
    pub fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
        // map
        // for strs
        // sort chars
        // add to map
        // for strs to gen result
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::default();
        for str in strs {
            let mut str_char = str.chars().collect::<Vec<char>>();
            str_char.sort();
            let sort_str: String = str_char.into_iter().collect();
            map.entry(sort_str).or_insert_with(Vec::new).push(str);
        }
        map.into_values().collect()
    }

    #[allow(dead_code)]
    pub fn group_anagrams3(strs: Vec<String>) -> Vec<Vec<String>> {
        // map
        // for strs
        // sort chars
        // add to map
        // for strs to gen result
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for str in strs {
            let mut str_char = str.chars().collect::<Vec<char>>();
            str_char.sort_unstable();
            map.entry(str_char).or_insert_with(Vec::new).push(str);
        }
        map.into_values().collect()
    }

    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // cal the other
        // get from map, if exist , return , else insert to map
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            let another = target - num;
            if mp.get(&another).is_none() {
                mp.insert(num, idx as i32);
            } else {
                let a_idx = mp.get(&another).unwrap().to_owned();
                return vec![a_idx, idx as i32];
            }
        }
        vec![-1, -1]
    }
}

use std::collections::BTreeMap;
struct MyCalendar {
    time_set: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            time_set: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((_, &p_end)) = self.time_set.range(..start_time).next_back() {
            if start_time < p_end {
                return false;
            }
        }
        if let Some((&n_start, _)) = self.time_set.range(start_time..).next() {
            if end_time > n_start {
                return false;
            }
        }

        self.time_set.insert(start_time, end_time);
        true
    }
}
