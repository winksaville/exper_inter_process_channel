use actor_bi_dir_channel::BiDirLocalChannel;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(RspAddActor "17a6ee73-6a91-42e2-908f-b1887e95d87a" {
    bdlc: Box<BiDirLocalChannel>
});

impl RspAddActor {
    pub fn new(bdlc: Box<BiDirLocalChannel>) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(RSP_ADD_ACTOR_ID),
            bdlc,
        }
    }
}
