use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{DashMap, mapref::one::Ref};

#[derive(Default, Debug, Clone)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.to_string()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn mget<T, K>(&self, table: &str, keys: T) -> Result<Vec<Kvpair>, KvError>
    where
        K: Into<String>,
        T: IntoIterator<Item = K>,
    {
        let table = self.get_or_create_table(table);
        let mut res: Vec<Kvpair> = Vec::new();
        for key in keys {
            let cur = table
                .get(key.into().as_str())
                .map(|v| Kvpair::new(v.key(), v.value().clone()));
            if let Some(v) = cur {
                res.push(v);
            }
        }
        Ok(res)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }

    fn mset(&self, table: &str, items: Vec<Kvpair>) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        for Kvpair { key, value } in items {
            let _ = table.insert(key, value.unwrap());
        }
        Ok(true)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn mdel<T, K>(&self, table: &str, keys: T) -> Result<bool, KvError>
    where
        K: Into<String>,
        T: IntoIterator<Item = K>,
    {
        let table = self.get_or_create_table(table);
        for key in keys {
            table.remove(&key.into()).unwrap();
        }
        Ok(true)
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // 查询出 dashMap
        let _table = self.get_or_create_table(table);

        // 返回迭代器
        todo!()
    }
}
