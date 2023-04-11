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
        dst_id: &AnId,
        src_id: &AnId,
        name: Option<&str>,
        id: Option<AnId>,
        protocol_id: Option<AnId>,
        protocol_set_id: Option<AnId>,
    ) -> Self {
        Self {
            header: MsgHeader::new(CON_MGR_QUERY_REQ_ID, *dst_id, *src_id),
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
    pub fn new(dst_id: &AnId, src_id: &AnId, instance_ids: &[AnId]) -> Self {
        Self {
            header: MsgHeader::new(CON_MGR_QUERY_RSP_ID, *dst_id, *src_id),
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

#[cfg(test)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_con_mgr_query_protocol() {
        let errp = con_mgr_query_protocol();
        assert_eq!(errp.id, CON_MGR_QUERY_PROTOCOL_ID);
        assert_eq!(errp.name, CON_MGR_QUERY_PROTOCOL_NAME);
        assert_eq!(errp.messages, *CON_MGR_QUERY_PROTOCOL_MESSAGES);
    }
}
