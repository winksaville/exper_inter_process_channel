use msg_macro::{msg_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_macro!(EchoReq "ada0f9a9-b556-46ba-b3d5-d19c87ec216c" {
    content: String
});

impl EchoReq {
    pub fn new(content: &str) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_REQ_ID },
            content: content.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_req_new() {
        let msg = EchoReq::new("hi");
        println!("test_echo_req msg={msg:?}");
        assert_eq!(msg.header.id, ECHO_REQ_ID);
        assert_eq!(&msg.content, "hi");
        assert_eq!(msg.header.id.to_string(), ECHO_REQ_ID_STR);
    }
}
