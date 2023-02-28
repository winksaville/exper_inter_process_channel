use actor_bi_dir_channel::BiDirLocalChannel;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(RspTheirBiDirChannel "03eb8c2c-0bc4-4d87-bcef-619e647b815f" {
    bdlc: Box<BiDirLocalChannel>
});

impl RspTheirBiDirChannel {
    pub fn new(bdlc: Box<BiDirLocalChannel>) -> Self {
        Self {
            header: msg_header::MsgHeader {
                id: RSP_THEIR_BI_DIR_CHANNEL_ID,
            },
            bdlc,
        }
    }
}
