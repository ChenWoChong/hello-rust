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
        let mut neighbors: Vec<_> = self
            .range
            .range(..=value + 1)
            .last()
            .into_iter()
            .chain(self.range.range(value..).next())
            .map(|(&k, &v)| (k, v))
            .collect();
        neighbors.dedup();

        let (mut start, mut end) = (value, value);

        for (ns, ne) in neighbors {
            if ns <= value && value <= ne {
                return;
            }

            if ne + 1 == value {
                start = ns;
                self.range.remove(&ns);
            }
            if ns - 1 == value {
                end = ne;
                self.range.remove(&ns);
            }
        }

        self.range.insert(start, end);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.range.iter().map(|(&p, &n)| vec![p, n]).collect()
    }
}
