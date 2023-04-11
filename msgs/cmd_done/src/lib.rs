use an_id::AnId;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(CmdDone "92a8798e-e2c9-493e-b863-edae4d302f14");

impl CmdDone {
    pub fn new(dst_id: &AnId, src_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(CMD_DONE_ID, *dst_id, *src_id),
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
        let msg = CmdDone::new(&dst_id, &src_id);
        println!("test_cmd_done_new msg={msg:?}");
        assert_eq!(msg.msg_id(), &CMD_DONE_ID);
        assert_eq!(msg.dst_id(), &dst_id);
        assert_eq!(msg.src_id(), &src_id);
        assert_eq!(msg.msg_id().to_string(), CMD_DONE_ID_STR);
    }
}
