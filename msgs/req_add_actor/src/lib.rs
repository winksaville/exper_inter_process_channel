use actor::Actor;
use an_id::AnId;
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro_not_cloneable, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro_not_cloneable!(ReqAddActor "8cc2afb6-c71f-43ae-a278-affcce76ffdd" {
    actor: Box<dyn Actor>
});

impl ReqAddActor {
    pub fn new(src_id: &AnId, actor: Box<dyn Actor>) -> Self {
        Self {
            header: MsgHeader::new(REQ_ADD_ACTOR_ID, *src_id),
            actor,
        }
    }
}
