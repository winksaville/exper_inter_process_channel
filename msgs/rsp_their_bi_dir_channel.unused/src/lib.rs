use actor_bi_dir_channel::BiDirLocalChannel;
use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(RspTheirBiDirChannel "03eb8c2c-0bc4-4d87-bcef-619e647b815f" {
    bdlc: Box<BiDirLocalChannel>
});

impl RspTheirBiDirChannel {
    pub fn new(src_id: &AnId, bdlc: Box<BiDirLocalChannel>) -> Self {
        Self {
            header: MsgHeader::new(RSP_THEIR_BI_DIR_CHANNEL_ID, *src_id),
            bdlc,
        }
    }
}
