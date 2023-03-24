use chrono::Utc;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(EchoReq "ada0f9a9-b556-46ba-b3d5-d19c87ec216c" {
    req_timestamp_ns: i64,
    counter: u64
});

impl EchoReq {
    pub fn new(counter: u64) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(ECHO_REQ_ID),
            req_timestamp_ns: Utc::now().timestamp_nanos(),
            counter,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_req_new() {
        let now_ns = Utc::now().timestamp_nanos();
        let msg = EchoReq::new(1);
        println!("test_echo_req msg={msg:?}");
        assert_eq!(msg.header.msg_id, ECHO_REQ_ID);
        // This isn't absolute true if the clock
        assert!(msg.req_timestamp_ns >= now_ns);
        assert_eq!(msg.counter, 1);
        assert_eq!(msg.header.msg_id.to_string(), ECHO_REQ_ID_STR);
    }
}
