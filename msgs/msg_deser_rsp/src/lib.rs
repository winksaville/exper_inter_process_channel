use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgDeserRspStatus {
    Success,
    MsgAlreadyRegistered,
}

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(MsgDeserRsp "6b0076ec-d404-43fd-a974-96320a6a093c" {
    msg_id: AnId,
    status: MsgDeserRspStatus
});

impl MsgDeserRsp {
    pub fn new(dst_id: &AnId, src_id: &AnId, msg_id: &AnId, status: MsgDeserRspStatus) -> Self {
        Self {
            header: MsgHeader::new(MSG_DESER_RSP_ID, *dst_id, *src_id),
            msg_id: *msg_id,
            status,
        }
    }
}
