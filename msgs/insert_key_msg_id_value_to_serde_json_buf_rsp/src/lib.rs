use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum InsertKeyMsgIdValueToSerdeJsonBufRspStatus {
    Success,
    AlreadyInserted,
}

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(InsertKeyMsgIdValueToSerdeJsonBufRsp "cf59aa4f-ff67-49ec-a48f-55bada1c4667" {
    msg_id: AnId,
    status: InsertKeyMsgIdValueToSerdeJsonBufRspStatus
});

impl InsertKeyMsgIdValueToSerdeJsonBufRsp {
    pub fn new(
        dst_id: &AnId,
        src_id: &AnId,
        msg_id: &AnId,
        status: InsertKeyMsgIdValueToSerdeJsonBufRspStatus,
    ) -> Self {
        Self {
            header: MsgHeader::new(
                INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_RSP_ID,
                *dst_id,
                *src_id,
            ),
            msg_id: *msg_id,
            status,
        }
    }
}
