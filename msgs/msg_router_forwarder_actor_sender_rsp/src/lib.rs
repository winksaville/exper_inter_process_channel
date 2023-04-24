use actor_channel::ActorSender;
use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(MsgRouterForwarderActorSenderRsp "124f9b7c-8a56-439e-b9fd-9f9ce03e8217" {
    sender: ActorSender
});

impl MsgRouterForwarderActorSenderRsp {
    pub fn new(dst_id: &AnId, src_id: &AnId, sender: &ActorSender) -> Self {
        Self {
            header: MsgHeader::new(MSG_ROUTER_FORWARDER_ACTOR_SENDER_RSP_ID, *dst_id, *src_id),
            sender: sender.clone(),
        }
    }
}
