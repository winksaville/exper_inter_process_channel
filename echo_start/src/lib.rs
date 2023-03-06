use actor_bi_dir_channel::ActorBiDirChannel;
use msg_local_macro::{paste, msg_local_macro_not_cloneable};

// https://www.uuidgenerator.net/version4
msg_local_macro_not_cloneable!(EchoStart "f13f7f26-40eb-4c94-a408-c455677f6730" {
    partner_abdc: Box<dyn ActorBiDirChannel + std::fmt::Debug>,
    ping_count: u64
});

impl EchoStart {
    pub fn new(partner_abdc: Box<dyn ActorBiDirChannel>, ping_count: u64) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_START_ID },
            partner_abdc,
            ping_count,
        }
    }
}
