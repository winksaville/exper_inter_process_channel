use actor::{ActorId, ActorInstanceId};
use msg_serde_macro::{msg_serde_macro, paste};
use protocol_set::ProtocolSet;

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(ConMgrAddActor "b0e83356-fd22-4389-9f2e-586be8ec9719" {
    name: String,
    id: ActorId,
    instance_id: ActorInstanceId,
    protocol_set: ProtocolSet
});

impl ConMgrAddActor {
    pub fn new(
        name: &str,
        id: &ActorId,
        instance_id: &ActorInstanceId,
        protocol_set: &ProtocolSet,
    ) -> Self {
        Self {
            header: msg_header::MsgHeader {
                id: CON_MGR_ADD_ACTOR_ID,
            },
            name: name.to_owned(),
            id: id.clone(),
            instance_id: instance_id.clone(),
            protocol_set: protocol_set.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use an_id::AnId;
    use msg_header::MsgId;
    use protocol::{Protocol, ProtocolId};
    use protocol_set::ProtocolSetId;

    use super::*;

    #[test]
    fn test_con_mgr_add_actor() {
        let a_id = ActorId(AnId::new());
        let a_instance_id = ActorInstanceId(AnId::new());

        let protocol1_id = ProtocolId(AnId::new());
        let protocol1_msgs = vec![MsgId(AnId::new())];
        let protocol1 = Protocol::new("protocol1", protocol1_id.clone(), protocol1_msgs.clone());

        let protocol_set_id = ProtocolSetId(AnId::new());
        let mut protocols_map = HashMap::new();
        assert!(protocols_map
            .insert(protocol1_id.clone(), protocol1.clone())
            .is_none());

        let a_protocol_set = ProtocolSet::new(
            "a_protocol_set",
            protocol_set_id.clone(),
            protocols_map.clone(),
        );

        let protocols = Vec::from_iter(a_protocol_set.protocols_map.values().into_iter());
        assert_eq!(protocols.len(), 1);

        let msg = ConMgrAddActor::new("cmaa1", &a_id, &a_instance_id, &a_protocol_set);
        println!("test_echo_req msg={msg:#?}");

        assert_eq!(msg.header.id, CON_MGR_ADD_ACTOR_ID);
        assert_eq!(msg.name, "cmaa1");
        assert_eq!(msg.id, a_id);
        assert_eq!(msg.instance_id, a_instance_id);

        assert_eq!(a_protocol_set.name, "a_protocol_set");
        assert_eq!(a_protocol_set.id, protocol_set_id);
        let p = a_protocol_set
            .protocols_map
            .get(&protocol1_id.clone())
            .unwrap();
        assert_eq!(p, &protocol1);
    }
}
