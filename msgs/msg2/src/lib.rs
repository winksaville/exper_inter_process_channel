use an_id::AnId;
use msg_header::MsgHeader;
use msg_serde_macro::{msg_serde_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(Msg2 "4029b3c4-f380-488a-8560-8320cc8fb76e");

impl Msg2 {
    pub fn new(dst_id: &AnId, src_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(MSG2_ID, *dst_id, *src_id),
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
        let msg = Msg2::new(&dst_id, &src_id);
        println!("test_cmd_done_new msg={msg:?}");
        assert_eq!(msg.msg_id(), &MSG2_ID);
        assert_eq!(msg.dst_id(), &dst_id);
        assert_eq!(msg.src_id(), &src_id);
        assert_eq!(msg.msg_id().to_string(), MSG2_ID_STR);
    }
}
