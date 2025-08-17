use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{DashMap, mapref::one::Ref};

#[derive(Default, Debug, Clone)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

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
        todo!()
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError> {
        todo!()
    }
}
