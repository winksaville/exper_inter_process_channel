use std::collections::HashMap;

use msg_header::MsgId;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ProtocolId(pub Uuid);

#[derive(Clone, Debug)]
pub struct ProtocolRec {
    name: String,
    id: ProtocolId,
    protocols: Vec<ProtocolId>,
    message_map: HashMap<ProtocolId, Vec<MsgId>>,
}

impl ProtocolRec {
    pub fn new(
        name: &str,
        id: ProtocolId,
        protocols: Vec<ProtocolId>,
        message_map: HashMap<ProtocolId, Vec<MsgId>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            id,
            protocols,
            message_map,
        }
    }
}

pub trait Protocol {
    fn name(&self) -> &str;
    fn id(&self) -> &ProtocolId;
    fn protocols(&self) -> &[ProtocolId];
    fn messages(&self, protocol_id: &ProtocolId) -> Option<&[MsgId]>;
}

impl Protocol for ProtocolRec {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &ProtocolId {
        &self.id
    }

    fn protocols(&self) -> &[ProtocolId] {
        &self.protocols
    }

    fn messages(&self, protocol_id: &ProtocolId) -> Option<&[MsgId]> {
        if let Some(v) = self.message_map.get(protocol_id) {
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_protocol() {
        println!("test_protocol");

        let messages = vec![MsgId(Uuid::new_v4())];
        let id = ProtocolId(Uuid::new_v4());
        let protocols = vec![id.clone()];
        let mut message_map = HashMap::new();
        message_map.insert(id.clone(), messages.clone());

        let a_protocol = ProtocolRec::new("a_protocol", id.clone(), protocols, message_map);

        println!("a_protocol={a_protocol:#?}");
        assert_eq!(a_protocol.name(), "a_protocol");
        assert_eq!(a_protocol.id(), &id);
        let msgs = a_protocol.message_map.get(&id).unwrap();
        assert_eq!(msgs, &messages);
    }
}
