use actor_bi_dir_channel::BiDirLocalChannel;
use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(RspAddActor "17a6ee73-6a91-42e2-908f-b1887e95d87a" {
    bdlc: Box<BiDirLocalChannel>
});

impl RspAddActor {
    pub fn new(src_id: &AnId, bdlc: Box<BiDirLocalChannel>) -> Self {
        Self {
            header: MsgHeader::new(RSP_ADD_ACTOR_ID, Some(*src_id)),
            bdlc,
        }
    }
}
