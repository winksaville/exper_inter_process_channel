use protocol::{Protocol, ProtocolId};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ProtocolSetId(pub Uuid);

#[derive(Clone, Debug)]
pub struct ProtocolSet {
    // Name of this ProtocolSet
    pub name: String,

    // Id for this ProtocolSet
    pub id: ProtocolSetId,

    /// HashMap from a ProtocolId to a Protocol
    pub protocols_map: HashMap<ProtocolId, Protocol>,
}

impl ProtocolSet {
    pub fn new(
        name: &str,
        id: ProtocolSetId,
        protocols_map: HashMap<ProtocolId, Protocol>,
    ) -> Self {
        Self {
            name: name.to_string(),
            id,
            protocols_map,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use msg_header::MsgId;
    use uuid::Uuid;

    #[test]
    fn test_protocol() {
        println!("test_protocol");

        let protocol1_id = ProtocolId(Uuid::new_v4());
        let protocol1_msgs = vec![MsgId(Uuid::new_v4())];
        let protocol1 = Protocol::new("protocol1", protocol1_id.clone(), protocol1_msgs.clone());

        let protocol2_id = ProtocolId(Uuid::new_v4());
        let protocol2_msgs = vec![MsgId(Uuid::new_v4())];
        let protocol2 = Protocol::new("protocol2", protocol2_id.clone(), protocol2_msgs.clone());

        let protocol_set_id = ProtocolSetId(Uuid::new_v4());
        let mut protocols_map = HashMap::new();
        assert!(protocols_map
            .insert(protocol1_id.clone(), protocol1.clone())
            .is_none());
        assert!(protocols_map
            .insert(protocol2_id.clone(), protocol2.clone())
            .is_none());

        let a_protocol_set = ProtocolSet::new(
            "a_protocol_set",
            protocol_set_id.clone(),
            protocols_map.clone(),
        );
        println!("a_protocol_set: {a_protocol_set:#?}");

        let protocols = Vec::from_iter(a_protocol_set.protocols_map.values().into_iter());
        assert_eq!(protocols.len(), 2);

        assert_eq!(a_protocol_set.id, protocol_set_id);
        assert_eq!(a_protocol_set.name, "a_protocol_set");
        let p = a_protocol_set
            .protocols_map
            .get(&protocol1_id.clone())
            .unwrap();
        assert_eq!(p, &protocol1);
        let p = a_protocol_set
            .protocols_map
            .get(&protocol2_id.clone())
            .unwrap();
        assert_eq!(p, &protocol2);
    }
}
