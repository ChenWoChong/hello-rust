use std::collections::HashMap;

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
