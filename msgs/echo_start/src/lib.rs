use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// https://www.uuidgenerator.net/version4
msg_local_macro!(EchoStart "f13f7f26-40eb-4c94-a408-c455677f6730" {
    partner_instance_id: AnId,
    ping_count: u64
});

impl EchoStart {
    pub fn new(partner_instance_id: &AnId, ping_count: u64) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(ECHO_START_ID),
            partner_instance_id: partner_instance_id.clone(),
            ping_count,
        }
    }
}
