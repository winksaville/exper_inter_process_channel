use an_id::AnId;
use msg_header::{FromSerdeJsonBuf, MsgHeader};
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(MsgDeserReq "fddef416-6314-4540-abd7-d8f6352fbb87" {
    msg_id: AnId,
    from_serde_json_buf: FromSerdeJsonBuf
});

impl MsgDeserReq {
    pub fn new(
        dst_id: &AnId,
        src_id: &AnId,
        msg_id: &AnId,
        from_serde_json_buf: FromSerdeJsonBuf,
    ) -> Self {
        Self {
            header: MsgHeader::new(MSG_DESER_REQ_ID, *dst_id, *src_id),
            msg_id: *msg_id,
            from_serde_json_buf,
        }
    }
}
