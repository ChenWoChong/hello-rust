mod command_service;

use crate::*;
pub use command_service::{Service, ServiceInner};

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::command_service::*;
    use crate::{MemTable, Value};
    use http::StatusCode;
    use std::thread::spawn;
    use tracing::info;

    #[test]
    fn service_should_works() {
        let service: Service = ServiceInner::new(MemTable::new()).into();

        let cloned = service.clone();

        let handle = spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);

        let res = service.execute(CommandRequest::new_hexist("t1", "k1"));
        assert_res_ok(res, &[], &[]);

        let res = service.execute(CommandRequest::new_hexist("t1", "k2"));
        assert_res_error(res, 500, "Not Found");
    }

    #[test]
    fn service_h_m_get_should_works() {
        let service: Service = ServiceInner::new(MemTable::new()).into();

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
        let service: Service = ServiceInner::new(MemTable::new()).into();

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
        let service: Service = ServiceInner::new(MemTable::new()).into();

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
        let service: Service = ServiceInner::new(MemTable::new()).into();

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

    #[test]
    fn event_registration_should_work() {
        fn b(cmd: &CommandRequest) {
            info!("Got CommandRequest: {:?}", cmd);
        }
        fn c(res: &CommandResponse) {
            info!("Got CommandResponse: {:?}", res);
        }

        fn d(res: &mut CommandResponse) {
            info!("Change CommandResponse: {:?}", res);
            res.status = StatusCode::CREATED.as_u16() as _
        }

        fn e() {
            info!("Data is sent");
        }

        let service: Service = ServiceInner::new(MemTable::new())
            .fn_received(|_| {})
            .fn_received(b)
            .fn_executed(c)
            .fn_before_send(d)
            .fn_after_send(e)
            .into();

        let res = service.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
        assert_eq!(res.status, StatusCode::CREATED.as_u16() as _);
        assert_eq!(res.message, "");
        assert_eq!(res.values, vec![Value::default()]);
    }
}
