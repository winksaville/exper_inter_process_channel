use an_id::AnId;
use chrono::Utc;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(EchoRsp "8206e26f-a69d-4875-8a85-0cfb636ca7c2" {
    req_timestamp_ns: i64,
    counter: u64,
    rsp_timestamp_ns: i64
});

impl EchoRsp {
    pub fn new(src_id: &AnId, req_timestamp: i64, counter: u64) -> Self {
        Self {
            header: MsgHeader::new(ECHO_RSP_ID, Some(src_id.clone())),
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
        let src_id = AnId::new();
        let now_ns = Utc::now().timestamp_nanos();
        let msg = EchoRsp::new(&src_id, now_ns, 1);
        println!("test_echo_rsp msg={msg:?}");
        assert_eq!(msg.header.msg_id, ECHO_RSP_ID);
        assert!(msg.rsp_timestamp_ns >= msg.req_timestamp_ns);
        assert_eq!(msg.counter, 1);
        assert_eq!(msg.header.msg_id.to_string(), ECHO_RSP_ID_STR);
        assert_eq!(msg.header.src_id, Some(src_id));
    }
}
