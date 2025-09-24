use crate::{KvError, Kvpair, Storage, Value};
use sled::{Db, Error, IVec};
use std::fmt::format;
use std::path::Path;

#[derive(Debug)]
pub struct SledDb(Db);

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    fn get_table_prefix(table: &str) -> String {
        format!("{}:", table)
    }
}

fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

impl Storage for SledDb {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = SledDb::get_full_key(table, key);
        let result = self.0.get(name.as_bytes()).map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn mget<T, K>(&self, table: &str, keys: T) -> Result<Vec<Kvpair>, KvError>
    where
        K: Into<String>,
        T: IntoIterator<Item = K>,
    {
        todo!()
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn mset(&self, table: &str, items: Vec<Kvpair>) -> Result<bool, KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn mdel<T, K>(&self, table: &str, keys: T) -> Result<bool, KvError>
    where
        K: Into<String>,
        T: IntoIterator<Item = K>,
    {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}

impl From<Result<Option<IVec>, sled::Error>> for Kvpair {
    fn from(value:Result<Option<IVec>, sled::Error>) -> Self {
        match value {
            Ok(v) => match v {
                Some(v) => Kvpair::new(ivec_to_key(v.as_ref())),
                None => Kvpair::default(),
            },
            _ => Kvpair::default(),
        }
    }
}

// impl From<Result<(IVec, IVec), sled::Error>> for Kvpair {
//     fn from(value: Result<(IVec, IVec), Error>) -> Self {
//         match value {
//             Ok((k, v)) => match v.as_ref().try_into() {
//                 Ok(v) => Kvpair::new(ivec_to_key(k.as_ref()), v),
//                 Err(_) => Kvpair::default(),
//             },
//             _ => Kvpair::default(),
//         }
//     }
// }

fn ivec_to_key(ivec: &[u8]) -> &str {
    let s = str::from_utf8(ivec).unwrap();
    let mut iter = s.split(":");
    iter.next();
    iter.next().unwrap()
}
