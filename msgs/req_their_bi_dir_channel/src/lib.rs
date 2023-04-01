use an_id::AnId;
use crossbeam_channel::Sender;
use msg_header::{BoxMsgAny, MsgHeader};
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(ReqTheirBiDirChannel "ea3145f6-078f-4085-9f86-30e27caca5e1" {
    handle: usize,
    rsp_tx: Sender<BoxMsgAny>
});

impl ReqTheirBiDirChannel {
    pub fn new(src_id: &AnId, handle: usize, rsp_tx: Sender<BoxMsgAny>) -> Self {
        Self {
            header: MsgHeader::new(REQ_THEIR_BI_DIR_CHANNEL_ID, Some(*src_id)),
            handle,
            rsp_tx,
        }
    }
}
