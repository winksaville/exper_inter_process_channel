use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(MsgRouterForwarderActorSenderReq "a3c8423e-f6be-4005-911e-8d4e6e21d442" {
    instance_id: AnId // Ignored for now
});

impl MsgRouterForwarderActorSenderReq {
    pub fn new(dst_id: &AnId, src_id: &AnId, instance_id: &AnId) -> Self {
        Self {
            header: MsgHeader::new(MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQ_ID, *dst_id, *src_id),
            instance_id: *instance_id,
        }
    }
}
