use chrono::Utc;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(EchoReply "8206e26f-a69d-4875-8a85-0cfb636ca7c2" {
    req_timestamp_ns: i64,
    counter: u64,
    reply_timestamp_ns: i64
});

impl EchoReply {
    pub fn new(req_timestamp: i64, counter: u64) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_REPLY_ID },
            req_timestamp_ns: req_timestamp,
            counter,
            reply_timestamp_ns: Utc::now().timestamp_nanos(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_reply_new() {
        let now_ns = Utc::now().timestamp_nanos();
        let msg = EchoReply::new(now_ns, 1);
        println!("test_echo_reply msg={msg:?}");
        assert_eq!(msg.header.id, ECHO_REPLY_ID);
        assert!(msg.reply_timestamp_ns >= msg.req_timestamp_ns);
        assert_eq!(msg.counter, 1);
        assert_eq!(msg.header.id.to_string(), ECHO_REPLY_ID_STR);
    }
}
