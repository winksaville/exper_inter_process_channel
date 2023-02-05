use msg_header::MsgId;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ProtocolId(pub Uuid);

#[allow(unused)]
#[derive(Clone, Debug)]
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

        let messages = vec![Uuid::nil(), Uuid::nil()];
        let id = ProtocolId(Uuid::nil());
        let nil_protocol = Protocol::new("nil_protocol", id, messages);
        println!("nil_protocol={nil_protocol:?}");
        assert_eq!(nil_protocol.name, "nil_protocol");
        assert_eq!(nil_protocol.id.0, Uuid::nil());
        assert_eq!(nil_protocol.messages, vec![Uuid::nil(), Uuid::nil()]);
    }
}
