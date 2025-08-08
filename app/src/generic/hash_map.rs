use itertools::Itertools;
use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet};

pub fn test_hash_map() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    map.shrink_to_fit();
    explain("shrinked", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    let capacity = map.capacity();
    let items = map.len();

    let bucket_mask = if capacity == 0 { 0 } else { capacity - 1 };

    println!(
        "[{name}]:\n\t(公开) capacity: {capacity}, len: {items},\t(推断) bucket_mask: 0x{bucket_mask:x}",
        name = name,
        items = items,
        bucket_mask = bucket_mask,
        capacity = capacity,
    );
}

#[derive(Ord, Debug, PartialEq, Eq, Hash, PartialOrd)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

pub fn test_map_key_name() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/user", 0x0), 28);

    for item in map.iter() {
        println!("{:?}", item);
    }
}

pub fn test_btree_map() {
    println!("-----test btree_map-----");
    let mut map = BTreeMap::new();
    for i in 0..16usize {
        map.insert(format!("chen {}", i), i);
        println!("Insert： chen {}", i);
    }

    map.remove("chen 1");
    println!("----------removed chen1----------");
    println!("----------Iter btree_map----------");
    for item in map.iter() {
        println!("{:?}", item);
    }
}

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
