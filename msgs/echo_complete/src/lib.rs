use an_id::AnId;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// https://www.uuidgenerator.net/version4
msg_serde_macro!(EchoComplete "d8c84131-901c-4900-b506-e4bac6665a58");

impl EchoComplete {
    pub fn new(src_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(ECHO_COMPLETE_ID, Some(*src_id)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_done_new() {
        let src_id = AnId::new();
        let msg = EchoComplete::new(&src_id);
        println!("test_cmd_done_new msg={msg:?}");
        assert_eq!(msg.header.msg_id, ECHO_COMPLETE_ID);
        assert_eq!(msg.header.src_id, Some(src_id));
        assert_eq!(msg.header.msg_id.to_string(), ECHO_COMPLETE_ID_STR);
    }
}
