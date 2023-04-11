use an_id::AnId;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(CmdInit "16119f69-17e9-4b3b-9b5c-eeac60af6056");

impl CmdInit {
    pub fn new(dst_id: &AnId, src_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(CMD_INIT_ID, *dst_id, *src_id),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_done_new() {
        let dst_id = AnId::new();
        let src_id = AnId::new();
        let msg = CmdInit::new(&dst_id, &src_id);
        println!("test_cmd_done_new msg={msg:?}");
        assert_eq!(msg.msg_id(), &CMD_INIT_ID);
        assert_eq!(msg.dst_id(), &dst_id);
        assert_eq!(msg.src_id(), &src_id);
        assert_eq!(msg.msg_id().to_string(), CMD_INIT_ID_STR);
    }
}
