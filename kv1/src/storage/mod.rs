mod memory;

use crate::{KvError, Kvpair, Value};

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }
}
