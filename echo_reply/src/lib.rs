use msg_macro::{msg_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_macro!(EchoReply "8206e26f-a69d-4875-8a85-0cfb636ca7c2" {
    content: String
});

impl EchoReply {
    pub fn new(content: &str) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_REPLY_ID },
            content: content.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_reply_new() {
        let msg = EchoReply::new("hello");
        println!("test_echo_reply msg={msg:?}");
        assert_eq!(msg.header.id, ECHO_REPLY_ID);
        assert_eq!(&msg.content, "hello");
        assert_eq!(msg.header.id.to_string(), ECHO_REPLY_ID_STR);
    }
}
