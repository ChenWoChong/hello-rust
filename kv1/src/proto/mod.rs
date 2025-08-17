use crate::proto::abi::command_request::RequestData;
pub use crate::proto::abi::{CommandRequest, Hset, Kvpair, Value, value};

mod abi;
use crate::KvError;
pub use abi::*;

impl CommandRequest {
    #[allow(dead_code)]
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self {
            value: Some(value::Value::String(v.into())),
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: 200,
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: 200,
            pairs: value,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let mut result = Self {
            status: 501,
            message: err.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match err {
            KvError::NotFound(_, _) => result.status = 404,
            KvError::InvalidCommand(_) => result.status = 504,
            _ => {}
        }
        result
    }
}
