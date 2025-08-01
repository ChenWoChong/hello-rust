use std::collections::HashMap;

pub fn test_hash_map() {
    let map = HashMap::new();
    let mut map = explain("empty", map);
    
    map.insert('a', 1);
    let mut map = explain("added 1", map);
    
    map.insert('b', 2);
    map.insert('c', 3);
    let mut map = explain("added 3", map);
    
}

fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "[{}]:\n\tbucket_mask 0x{:x}, ctrl px{:x}, grown_left: {}, items: {}",
        name, arr[2], arr[3], arr[4], arr[5]
    );
    unsafe { std::mem::transmute(arr) }
}
