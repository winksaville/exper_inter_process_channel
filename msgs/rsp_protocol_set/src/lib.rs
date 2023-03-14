use an_id::AnId;
use msg_serde_macro::{msg_serde_macro, paste};
use protocol_set::ProtocolSet;

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(RspProtocolSet "e7cced7e-5668-44ff-a2df-ad8073136f8b" {
    name: String,
    id: AnId,
    instance_id: AnId,
    protocol_set: ProtocolSet
});

impl RspProtocolSet {
    pub fn new(name: &str, id: &AnId, instance_id: &AnId, protocol_set: &ProtocolSet) -> Self {
        Self {
            header: msg_header::MsgHeader {
                id: RSP_PROTOCOL_SET_ID,
            },
            name: name.to_owned(),
            id: *id,
            instance_id: *instance_id,
            protocol_set: protocol_set.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use an_id::AnId;
    use protocol::Protocol;

    use super::*;

    #[test]
    fn test_rsp_protocol_set_msg() {
        let a_id = AnId::new();
        let a_instance_id = AnId::new();

        let protocol1_id = AnId::new();
        let protocol1_msgs = vec![AnId::new()];
        let protocol1 = Protocol::new("protocol1", protocol1_id.clone(), protocol1_msgs.clone());

        let protocol_set_id = AnId::new();
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

        let msg = RspProtocolSet::new("rps1", &a_id, &a_instance_id, &a_protocol_set);
        println!("test_echo_req msg={msg:#?}");

        assert_eq!(msg.header.id, RSP_PROTOCOL_SET_ID);
        assert_eq!(msg.name, "rps1");
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
