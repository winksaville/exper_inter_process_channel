//! Defines the two protocols for registering actors with
//! the connection manager. The ConMgr is the "Register"
//! and the actor requesting to be registered is the
//! "Registee".
//!
//! The CON_MGR_REGISTER_ACTOR_PROTOCOL is implemented by
//! the connection manager and it receives CON_MGR_REGISTER_ACTOR_REQ_ID
//! messages and it sends CON_MGR_REGISTER_ACTOR_RSP_ID messages.
//!
//! The CON_MGR_REGISTEE_ACTOR_PROTOCOL is implemented by
//! the actors that want to register with the connection manager
//! they send CON_MGR_REGISTER_ACTOR_RSP_ID messages.
use an_id::{anid, AnId};
use msg_header::MsgHeader;
use msg_local_macro::{msg_local_macro, paste};
use once_cell::sync::Lazy;
use protocol::Protocol;
use protocol_set::ProtocolSet;
use serde::{Deserialize, Serialize};

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(ConMgrRegisterActorReq "b0e83356-fd22-4389-9f2e-586be8ec9719" {
    name: String,
    id: AnId,
    instance_id: AnId,
    //protocol_set_id: AnId,
    // As ProtocolSet is immutable, see is #13
    //   https://github.com/winksaville/exper_inter_process_channel/issues/13
    protocol_set: ProtocolSet // TODO maybe make Option<ProtocolSet>
});

impl ConMgrRegisterActorReq {
    pub fn new(
        src_id: &AnId,
        name: &str,
        id: &AnId,
        instance_id: &AnId,
        protocol_set: &ProtocolSet,
    ) -> Self {
        Self {
            header: MsgHeader::new(CON_MGR_REGISTER_ACTOR_REQ_ID, Some(*src_id)),
            name: name.to_owned(),
            id: *id,
            instance_id: *instance_id,
            protocol_set: protocol_set.clone(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ConMgrRegisterActorStatus {
    Success,
    ActorAlreadyRegistered,
}

// From: https://www.uuidgenerator.net/version4
msg_local_macro!(ConMgrRegisterActorRsp "db6a401d-cd0a-4585-8ac4-c13ae1ab7a39" {
    // Should we add a transaction id here and in ConMgrReqActor?
    status: ConMgrRegisterActorStatus
});

impl ConMgrRegisterActorRsp {
    pub fn new(src_id: &AnId, status: ConMgrRegisterActorStatus) -> Self {
        Self {
            header: MsgHeader::new(CON_MGR_REGISTER_ACTOR_RSP_ID, Some(*src_id)),
            status,
        }
    }
}

static CON_MGR_REGISTER_ACTOR_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![CON_MGR_REGISTER_ACTOR_REQ_ID, CON_MGR_REGISTER_ACTOR_RSP_ID]);

// From: https://www.uuidgenerator.net/version4
const CON_MGR_REGISTER_ACTOR_PROTOCOL_ID: AnId = anid!("66fa196c-3871-4139-86b3-f98bc9d2dfe7");
const CON_MGR_REGISTER_ACTOR_PROTOCOL_NAME: &str = "con_mgr_register_actor_protocol";
static CON_MGR_REGISTER_ACTOR_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CON_MGR_REGISTER_ACTOR_PROTOCOL_NAME,
        CON_MGR_REGISTER_ACTOR_PROTOCOL_ID,
        CON_MGR_REGISTER_ACTOR_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn con_mgr_register_actor_protocol() -> &'static Protocol {
    &CON_MGR_REGISTER_ACTOR_PROTOCOL
}

// From: https://www.uuidgenerator.net/version4
const CON_MGR_REGISTEE_ACTOR_PROTOCOL_ID: AnId = anid!("fcaa554c-6969-42a3-841f-703bd18d93c4");
const CON_MGR_REGISTEE_ACTOR_PROTOCOL_NAME: &str = "con_mgr_register_actor_protocol";
static CON_MGR_REGISTEE_ACTOR_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CON_MGR_REGISTEE_ACTOR_PROTOCOL_NAME,
        CON_MGR_REGISTEE_ACTOR_PROTOCOL_ID,
        CON_MGR_REGISTER_ACTOR_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn con_mgr_registee_actor_protocol() -> &'static Protocol {
    &CON_MGR_REGISTEE_ACTOR_PROTOCOL
}

#[cfg(test)]
#[cfg(test)]
mod test {
    use super::*;

    use box_msg_any::BoxMsgAny;
    use crossbeam_channel::unbounded;
    use echo_requestee_protocol::echo_requestee_protocol;
    use std::collections::HashMap;

    #[test]
    fn test_con_mgr_reg_actor_protocol() {
        let errp = con_mgr_register_actor_protocol();
        assert_eq!(errp.id, CON_MGR_REGISTER_ACTOR_PROTOCOL_ID);
        assert_eq!(errp.name, CON_MGR_REGISTER_ACTOR_PROTOCOL_NAME);
        assert_eq!(errp.messages, *CON_MGR_REGISTER_ACTOR_PROTOCOL_MESSAGES);
    }

    #[test]
    fn test_con_mgr_reg_actor_protocol_set() {
        let a_src_id = AnId::new();
        let a_id = AnId::new();
        let a_instance_id = AnId::new();
        let a_protocol_set_id = AnId::new();

        // A Protocol set with echo requestee
        let erep = echo_requestee_protocol();
        let mut pm = HashMap::<AnId, Protocol>::new();
        pm.insert(erep.id.clone(), erep.clone());
        let ps = ProtocolSet::new("ps", a_protocol_set_id, pm);

        let msg = ConMgrRegisterActorReq::new(&a_src_id, "cmra1", &a_id, &a_instance_id, &ps);
        println!("test_con_mgr_reg_actor_protocol_set_some: msg={msg:#?}");

        let (theirs, ours) = unbounded::<BoxMsgAny>();

        assert_eq!(msg.header.msg_id, CON_MGR_REGISTER_ACTOR_REQ_ID);
        assert_eq!(msg.header.src_id, Some(a_src_id));
        assert_eq!(msg.name, "cmra1");
        assert_eq!(msg.id, a_id);
        assert_eq!(msg.instance_id, a_instance_id);
        assert_eq!(msg.protocol_set, ps);

        println!("sending");
        theirs.send(Box::new(msg)).unwrap();
        println!("receiving");
        let msg_any = ours.recv().unwrap();
        println!("received");
        let msg_rcvd = msg_any.downcast::<ConMgrRegisterActorReq>().unwrap();

        assert_eq!(msg_rcvd.header.msg_id, CON_MGR_REGISTER_ACTOR_REQ_ID);
        assert_eq!(msg_rcvd.header.src_id, Some(a_src_id));
        assert_eq!(msg_rcvd.name, "cmra1");
        assert_eq!(msg_rcvd.id, a_id);
        assert_eq!(msg_rcvd.instance_id, a_instance_id);
        assert_eq!(msg_rcvd.protocol_set, ps);
    }

    //#[test]
    //fn test_con_mgr_reg_actor_protocol_set_none() {
    //    let a_id = AnId::new();
    //    let a_instance_id = AnId::new();
    //    let a_protocol_set_id = AnId::new();

    //    let (theirs, _ours) = BiDirLocalChannel::new();

    //    let msg = ConMgrRegisterActorReq::new(
    //        "cmra1",
    //        &a_id,
    //        &a_instance_id,
    //        &a_protocol_set_id,
    //        None,
    //        theirs,
    //    );
    //    println!("test_con_mgr_reg_actor_protocol_set_none: msg={msg:#?}");

    //    assert_eq!(msg.header.id, CON_MGR_REGISTER_ACTOR_REQ_ID);
    //    assert_eq!(msg.name, "cmra1");
    //    assert_eq!(msg.id, a_id);
    //    assert_eq!(msg.instance_id, a_instance_id);
    //    assert_eq!(msg.protocol_set_id, a_protocol_set_id);
    //    assert!(msg.protocol_set.is_none());

    //    let msg = ConMgrRegisterActorRsp::new(ConMgrRegisterActorStatus::Success);
    //    assert_eq!(msg.header.id, CON_MGR_REGISTER_ACTOR_RSP_ID);
    //    assert_eq!(msg.status, ConMgrRegisterActorStatus::Success);
    //}

    //#[test]
    //fn test_con_mgr_reg_actor_protocol_set_some() {
    //    let a_id = AnId::new();
    //    let a_instance_id = AnId::new();
    //    let a_protocol_set_id = AnId::new();

    //    // A Protocol set with echo requestee
    //    let erep = echo_requestee_protocol();
    //    let mut pm = HashMap::<AnId, Protocol>::new();
    //    pm.insert(erep.id.clone(), erep.clone());
    //    let ps = ProtocolSet::new("ps", a_protocol_set_id, pm);

    //    let (theirs, ours) = BiDirLocalChannel::new();

    //    let msg = ConMgrRegisterActorReq::new(
    //        "cmra1",
    //        &a_id,
    //        &a_instance_id,
    //        &a_protocol_set_id,
    //        Some(&ps),
    //        theirs,
    //    );
    //    println!("test_con_mgr_reg_actor_protocol_set_some: msg={msg:#?}");

    //    assert_eq!(msg.header.id, CON_MGR_REGISTER_ACTOR_REQ_ID);
    //    assert_eq!(msg.name, "cmra1");
    //    assert_eq!(msg.id, a_id);
    //    assert_eq!(msg.instance_id, a_instance_id);
    //    assert_eq!(msg.protocol_set_id, a_protocol_set_id);
    //    assert_eq!(msg.protocol_set.is_some(), true);
    //    assert_eq!(msg.protocol_set.unwrap(), ps);
    //    let their_channel: Box<dyn ActorBiDirChannel> = Box::new(msg.bdlc.clone());
    //    println!("sending");
    //    their_channel.send(Box::new(1)).unwrap();
    //    println!("receiving");
    //    let msg_any = ours.recv().unwrap();
    //    println!("received");
    //    assert_eq!(*msg_any.downcast::<i32>().unwrap(), 1);
    //}
}
