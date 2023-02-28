use actor::Actor;
use crossbeam_channel::Sender;
use msg_header::BoxMsgAny;
use msg_local_macro::{msg_local_macro_not_cloneable, paste};

// From: https://www.uuidgenerator.net/version4
msg_local_macro_not_cloneable!(ReqAddActor "8cc2afb6-c71f-43ae-a278-affcce76ffdd" {
    actor: Box<dyn Actor>,
    rsp_tx: Sender<BoxMsgAny>
});

impl ReqAddActor {
    pub fn new(actor: Box<dyn Actor>, rsp_tx: Sender<BoxMsgAny>) -> Self {
        Self {
            header: msg_header::MsgHeader {
                id: REQ_ADD_ACTOR_ID,
            },
            actor,
            rsp_tx,
        }
    }
}
