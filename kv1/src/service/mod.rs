mod command_service;

use crate::*;
pub use command_service::Service;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::command_service::*;
    use crate::{MemTable, Value};
    use std::thread::spawn;

    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }

    #[test]
    fn service_h_m_get_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
            let res = cloned.execute(CommandRequest::new_hset("t1", "k2", "v2".into()));
            assert_res_ok(res, &[Value::default()], &[]);
            let res = cloned.execute(CommandRequest::new_hset("t1", "k3", "v3".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hmget("t1", ["k1", "k2", "k3"]));
        assert_res_ok(
            res,
            &[],
            &[
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
                Kvpair::new("k3", "v3".into()),
            ],
        )
    }

    #[test]
    fn service_h_m_set_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hmset(
                "t1",
                vec![
                    Kvpair::new("k1", "v1".into()),
                    Kvpair::new("k2", "v2".into()),
                    Kvpair::new("k3", "v3".into()),
                ],
            ));
            assert_res_ok(res, &[], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hmget("t1", ["k1", "k2", "k3"]));
        assert_res_ok(
            res,
            &[],
            &[
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
                Kvpair::new("k3", "v3".into()),
            ],
        )
    }

    #[test]
    fn service_h_del_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hmset(
                "t2",
                vec![
                    Kvpair::new("k1", "v1".into()),
                    Kvpair::new("k2", "v2".into()),
                    Kvpair::new("k3", "v3".into()),
                ],
            ));
            assert_res_ok(res, &[], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hdel("t2", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);

        let res = service.execute(CommandRequest::new_hmget("t2", ["k1", "k2", "k3"]));
        assert_res_ok(
            res,
            &[],
            &[
                Kvpair::new("k2", "v2".into()),
                Kvpair::new("k3", "v3".into()),
            ],
        )
    }

    #[test]
    fn service_h_m_del_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hmset(
                "t2",
                vec![
                    Kvpair::new("k1", "v1".into()),
                    Kvpair::new("k2", "v2".into()),
                    Kvpair::new("k3", "v3".into()),
                    Kvpair::new("k4", "v4".into()),
                    Kvpair::new("k5", "v5".into()),
                    Kvpair::new("k6", "v6".into()),
                ],
            ));
            assert_res_ok(res, &[], &[]);
        });
        handle.join().unwrap();

        let cloned = service.clone();
        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hmdel("t2", ["k1", "k2", "k3"]));
            assert_res_ok(res, &[], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hmget(
            "t2",
            ["k1", "k2", "k3", "k4", "k5", "k6"],
        ));
        assert_res_ok(
            res,
            &[],
            &[
                Kvpair::new("k4", "v4".into()),
                Kvpair::new("k5", "v5".into()),
                Kvpair::new("k6", "v6".into()),
            ],
        )
    }
}
