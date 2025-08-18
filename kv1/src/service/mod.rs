mod command_service;

use crate::*;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::command_service::*;
    use crate::{MemTable, Value};
    use std::thread;

    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

