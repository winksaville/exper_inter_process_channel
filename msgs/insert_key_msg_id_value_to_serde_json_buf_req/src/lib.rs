use an_id::AnId;
use msg_header::{MsgHeader, ToSerdeJsonBuf};
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(InsertKeyMsgIdValueToSerdeJsonBufReq "3610bfb2-9636-4129-bc9f-67ff0c88c3c8" {
    msg_id: AnId,
    to_serde_json_buf: ToSerdeJsonBuf
});

impl InsertKeyMsgIdValueToSerdeJsonBufReq {
    pub fn new(
        dst_id: &AnId,
        src_id: &AnId,
        msg_id: &AnId,
        to_serde_json_buf: ToSerdeJsonBuf,
    ) -> Self {
        Self {
            header: MsgHeader::new(
                INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQ_ID,
                *dst_id,
                *src_id,
            ),
            msg_id: *msg_id,
            to_serde_json_buf,
        }
    }
}
