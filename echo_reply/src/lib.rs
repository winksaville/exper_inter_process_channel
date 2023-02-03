use echo_req::EchoReq;
use msg_macro::{msg_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_macro!(EchoReply "8206e26f-a69d-4875-8a85-0cfb636ca7c2" {
    content: String,
    counter: u64
});

impl EchoReply {
    pub fn new(content: &str, counter: u64) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_REPLY_ID },
            content: content.to_string(),
            counter,
        }
    }
    pub fn from_echo_req(msg: &EchoReq) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_REPLY_ID },
            content: msg.content.clone(),
            counter: msg.counter + 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_reply_new() {
        let msg = EchoReply::new("hello", 1);
        println!("test_echo_reply msg={msg:?}");
        assert_eq!(msg.header.id, ECHO_REPLY_ID);
        assert_eq!(&msg.content, "hello");
        assert_eq!(msg.counter, 1);
        assert_eq!(msg.header.id.to_string(), ECHO_REPLY_ID_STR);
    }

    #[test]
    fn test_echo_reply_from() {
        let msg_echo_req = EchoReq::new("hi", 123);
        println!("test_echo_reply msg_echo_req={msg_echo_req:?}");
        let msg_echo_reply = EchoReply::from_echo_req(&msg_echo_req);
        println!("test_echo_reply msg_echo_reply={msg_echo_reply:?}");
        assert_eq!(msg_echo_reply.header.id, ECHO_REPLY_ID);
        assert_eq!(&msg_echo_reply.content, "hi");
        assert_eq!(msg_echo_reply.counter, 124);
        assert_eq!(msg_echo_reply.header.id.to_string(), ECHO_REPLY_ID_STR);
    }
}
