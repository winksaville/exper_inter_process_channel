use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(RspAddActor "17a6ee73-6a91-42e2-908f-b1887e95d87a" {
    actor_id: AnId,
    actor_instance_id: AnId
});

impl RspAddActor {
    pub fn new(src_id: &AnId, actor_id: &AnId, actor_instance_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(RSP_ADD_ACTOR_ID, *src_id),
            actor_id: *actor_id,
            actor_instance_id: *actor_instance_id,
        }
    }
}
