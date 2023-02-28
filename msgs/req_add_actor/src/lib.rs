////use actor::Actor;
//use crossbeam_channel::Sender;
//use msg_header::BoxMsgAny;
//use msg_local_macro::{msg_local_macro, paste};
//
//// From: https://www.uuidgenerator.net/version4
//msg_local_macro!(ReqAddActor "8cc2afb6-c71f-43ae-a278-affcce76ffdd" {
//    //actor: Box<dyn Actor>,
//    rsp_tx: Sender<BoxMsgAny>
//});

use actor::Actor;
use msg_header::{BoxMsgAny, MsgHeader, MsgId};
use uuid::uuid;
use crossbeam_channel::Sender;

// From: https://www.uuidgenerator.net/version4
pub const REQ_ADD_ACTOR_ID_STR: &str = "8cc2afb6-c71f-43ae-a278-affcce76ffdd";
pub const REQ_ADD_ACTOR_ID: MsgId = MsgId(uuid!("8cc2afb6-c71f-43ae-a278-affcce76ffdd"));
pub const REQ_ADD_ACTOR_NAME: &str = "ReqAddActor";

// Message 1
//#[derive(Debug) //, Clone, PartialEq, Eq)]
#[derive(Debug)]
#[repr(C)]
pub struct ReqAddActor {
    pub header: MsgHeader,
    pub actor: Box<dyn Actor>,
    pub rsp_tx: Sender<BoxMsgAny>,
}

impl ReqAddActor {
    pub fn new(actor: Box<dyn Actor>, tx: Sender<BoxMsgAny>) -> Self {
        Self {
            header: MsgHeader { id: REQ_ADD_ACTOR_ID },
            actor,
            rsp_tx: tx,
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }

    pub fn from_box_msg_any(msg: &BoxMsgAny) -> Option<&ReqAddActor> {
        if let Some(msg) = msg.downcast_ref::<Self>() {
            Some(msg)
        } else {
            None
        }
    }
}
