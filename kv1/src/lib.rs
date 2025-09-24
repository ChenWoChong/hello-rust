mod error;
mod proto;
mod service;
mod storage;

pub use error::*;
pub use proto::*;
pub use service::*;
pub use storage::{MemTable, SledDb, Storage};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
