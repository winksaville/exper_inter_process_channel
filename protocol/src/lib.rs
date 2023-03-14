use an_id::AnId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Protocol {
    pub name: String,
    pub id: AnId,
    pub messages: Vec<AnId>,
}

impl Protocol {
    pub fn new(name: &str, id: AnId, messages: Vec<AnId>) -> Self {
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

        let id = AnId::new();
        let messages = vec![AnId::new()];
        let a_protocol = Protocol::new("a_protocol", id.clone(), messages.clone());

        println!("a_protocol={a_protocol:#?}");
        assert_eq!(a_protocol.name, "a_protocol");
        assert_eq!(a_protocol.id, id);
        assert_eq!(a_protocol.messages, messages);
    }
}
