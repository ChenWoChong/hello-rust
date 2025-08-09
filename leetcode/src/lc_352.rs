use std::collections::BTreeMap;
struct SummaryRanges {
    range: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        Self {
            range: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let prev = self
            .range
            .range(..=value)
            .next_back()
            .map(|(&start, &end)| (start, end));
        let next = self
            .range
            .range(value..)
            .next()
            .map(|(&start, &end)| (start, end));

        if let Some((_, end)) = prev {
            if value <= end {
                return;
            }
        }

        let can_merge_prev = prev.map_or(false, |(_, end)| end + 1 == value);
        let can_merge_next = next.map_or(false, |(start, _)| start - 1 == value);

        match (can_merge_prev, can_merge_next) {
            (true, true) => {
                let (pre_start, _) = prev.unwrap();
                let (next_start, next_end) = next.unwrap();

                // 无需 remove ，直接即可更新
                // self.range.remove(&pre_start);
                self.range.remove(&next_start);
                self.range.insert(pre_start, next_end);
            }
            (true, false) => {
                let (pre_start, _) = prev.unwrap();
                // 无需 remove ，直接即可更新
                // self.range.remove(&pre_start);
                self.range.insert(pre_start, value);
            }
            (false, true) => {
                let (next_start, next_end) = next.unwrap();
                self.range.remove(&next_start);
                self.range.insert(value, next_end);
            }
            (false, false) => {
                self.range.insert(value, value);
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.range.iter().map(|(&p, &n)| vec![p, n]).collect()
    }
}
