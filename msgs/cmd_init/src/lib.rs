use an_id::AnId;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(CmdInit "16119f69-17e9-4b3b-9b5c-eeac60af6056");

impl CmdInit {
    pub fn new(src_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(CMD_INIT_ID, *src_id),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_done_new() {
        let src_id = AnId::new();
        let msg = CmdInit::new(&src_id);
        println!("test_cmd_done_new msg={msg:?}");
        assert_eq!(msg.header.msg_id, CMD_INIT_ID);
        assert_eq!(msg.header.src_id, src_id);
        assert_eq!(msg.header.msg_id.to_string(), CMD_INIT_ID_STR);
    }
}
