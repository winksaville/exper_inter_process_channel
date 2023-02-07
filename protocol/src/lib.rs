use msg_header::MsgId;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ProtocolId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Protocol {
    pub name: String,
    pub id: ProtocolId,
    pub messages: Vec<MsgId>,
}

impl Protocol {
    pub fn new(name: &str, id: ProtocolId, messages: Vec<MsgId>) -> Self {
        Self {
            name: name.to_string(),
            id,
            messages,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_protocol() {
        println!("test_protocol");

        let id = ProtocolId(Uuid::new_v4());
        let messages = vec![MsgId(Uuid::new_v4())];
        let a_protocol = Protocol::new("a_protocol", id.clone(), messages.clone());

        println!("a_protocol={a_protocol:#?}");
        assert_eq!(a_protocol.name, "a_protocol");
        assert_eq!(a_protocol.id, id);
        assert_eq!(a_protocol.messages, messages);
    }
}
