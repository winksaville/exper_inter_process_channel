use chrono::Utc;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(EchoRsp "8206e26f-a69d-4875-8a85-0cfb636ca7c2" {
    req_timestamp_ns: i64,
    counter: u64,
    rsp_timestamp_ns: i64
});

impl EchoRsp {
    pub fn new(req_timestamp: i64, counter: u64) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_RSP_ID },
            req_timestamp_ns: req_timestamp,
            counter,
            rsp_timestamp_ns: Utc::now().timestamp_nanos(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_rsp_new() {
        let now_ns = Utc::now().timestamp_nanos();
        let msg = EchoRsp::new(now_ns, 1);
        println!("test_echo_rsp msg={msg:?}");
        assert_eq!(msg.header.id, ECHO_RSP_ID);
        assert!(msg.rsp_timestamp_ns >= msg.req_timestamp_ns);
        assert_eq!(msg.counter, 1);
        assert_eq!(msg.header.id.to_string(), ECHO_RSP_ID_STR);
    }
}