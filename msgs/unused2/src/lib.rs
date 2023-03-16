use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(ReqProtocolSet "a3c8423e-f6be-4005-911e-8d4e6e21d442");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_req_protocol_set_msg() {
        let msg = ReqProtocolSet::new();
        println!("test_req_protocol_set_msg: msg={msg:?}");
        assert_eq!(msg.header.id, REQ_PROTOCOL_SET_ID);
    }
}
