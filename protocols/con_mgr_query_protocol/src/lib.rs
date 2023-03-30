//! Protocol for connecting actors.
//!
//! To keep things simple and controlled you may only
//! connect using an InstanceId. So you must first do a ConMgrQueryReq
//! for applicable actors using "name", "id", "protoocol"
//! and/or "protocol_set" and then use ConMgrConnectReq with an instance_id
//! retunred by ConMgrQueryRsp.
use an_id::{anid, AnId};
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};
use once_cell::sync::Lazy;
use protocol::Protocol;

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(ConMgrQueryReq "e5922aa5-9731-4379-813b-8cf5d0319d3d" {
    name: Option<String>,
    id: Option<AnId>,
    protocol_id: Option<AnId>,
    protocol_set_id: Option<AnId>
});

impl ConMgrQueryReq {
    pub fn new(
        name: Option<&str>,
        id: Option<AnId>,
        protocol_id: Option<AnId>,
        protocol_set_id: Option<AnId>,
    ) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(CON_MGR_QUERY_REQ_ID),
            name: name.map(|s| s.to_owned()),
            id,
            protocol_id,
            protocol_set_id,
        }
    }
}

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(ConMgrQueryRsp "162306ca-10b5-4bc9-9537-f7d8c53c7d0a" {
    instance_ids: Vec<AnId>
});

impl ConMgrQueryRsp {
    pub fn new(instance_ids: &[AnId]) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(CON_MGR_QUERY_RSP_ID),
            instance_ids: instance_ids.to_owned(),
        }
    }
}

static CON_MGR_QUERY_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![CON_MGR_QUERY_REQ_ID, CON_MGR_QUERY_RSP_ID]);

// From: https://www.uuidgenerator.net/version4
const CON_MGR_QUERY_PROTOCOL_ID: AnId = anid!("0b22d500-f51f-421f-bf59-2b553f47c459");
const CON_MGR_QUERY_PROTOCOL_NAME: &str = "con_mgr_query_protocol";
static CON_MGR_QUERY_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CON_MGR_QUERY_PROTOCOL_NAME,
        CON_MGR_QUERY_PROTOCOL_ID,
        CON_MGR_QUERY_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn con_mgr_query_protocol() -> &'static Protocol {
    &CON_MGR_QUERY_PROTOCOL
}

//// From: https://www.uuidgenerator.net/version4
//msg_local_macro!(ConMgrConnectReq "94757aed-87bc-4b8a-bbd6-e4ac4ca04233" {
//    instance_id: AnId,
//    their_bdlc_with_us: BiDirLocalChannel
//});
//
//impl ConMgrConnectReq {
//    pub fn new(instance_id: &AnId, their_bdlc_with_us: &BiDirLocalChannel) -> Self {
//        Self {
//            header: MsgHeader::new_msg_id_only(CON_MGR_CONNECT_REQ_ID),
//            instance_id: *instance_id,
//            their_bdlc_with_us: their_bdlc_with_us.clone(),
//        }
//    }
//}
//
//#[repr(C)]
//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
//pub enum ConMgrConnectStatus {
//    Success,
//    Rejected,
//    ActorAlreadyConnected,
//    RejectedToManyConnections,
//}
//
//// From: https://www.uuidgenerator.net/version4
//msg_local_macro!(ConMgrConnectRsp "8d735bca-1be0-4c66-b52f-2a18bb310bcf" {
//    status: ConMgrConnectStatus,
//    partner_tx: Option<Sender<BoxMsgAny>>
//});
//
//impl ConMgrConnectRsp {
//    pub fn new(status: ConMgrConnectStatus, partner_tx: Option<Sender<BoxMsgAny>>) -> Self {
//        assert_eq!(status == ConMgrConnectStatus::Success, partner_tx.is_some());
//        Self {
//            header: MsgHeader::new_msg_id_only(CON_MGR_CONNECT_RSP_ID),
//            status,
//            partner_tx,
//        }
//    }
//}

static CON_MGR_CONNECT_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> = Lazy::new(|| {
    vec![
        //CON_MGR_CONNECT_REQ_ID,
        //CON_MGR_CONNECT_RSP_ID,
        CON_MGR_QUERY_REQ_ID,
        CON_MGR_QUERY_RSP_ID,
    ]
});

// From: https://www.uuidgenerator.net/version4
const CON_MGR_CONNECT_PROTOCOL_ID: AnId = anid!("418f7430-835d-46d9-bf3f-f66a53f3d0b6");
const CON_MGR_CONNECT_PROTOCOL_NAME: &str = "con_mgr_connect_protocol";
static CON_MGR_CONNECT_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CON_MGR_CONNECT_PROTOCOL_NAME,
        CON_MGR_CONNECT_PROTOCOL_ID,
        CON_MGR_CONNECT_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn con_mgr_connect_protocol() -> &'static Protocol {
    &CON_MGR_CONNECT_PROTOCOL
}

#[cfg(test)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_con_mgr_connect_protocol() {
        let errp = con_mgr_connect_protocol();
        assert_eq!(errp.id, CON_MGR_CONNECT_PROTOCOL_ID);
        assert_eq!(errp.name, CON_MGR_CONNECT_PROTOCOL_NAME);
        assert_eq!(errp.messages, *CON_MGR_CONNECT_PROTOCOL_MESSAGES);
    }

    #[test]
    fn test_con_mgr_query_protocol() {
        let errp = con_mgr_query_protocol();
        assert_eq!(errp.id, CON_MGR_QUERY_PROTOCOL_ID);
        assert_eq!(errp.name, CON_MGR_QUERY_PROTOCOL_NAME);
        assert_eq!(errp.messages, *CON_MGR_QUERY_PROTOCOL_MESSAGES);
    }
}
