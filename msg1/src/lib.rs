use msg_header::{MsgHeader, MsgId};
use serde::{Deserialize, Serialize};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG1_ID: MsgId = uuid!("a88ba7e7-0930-4df6-bb24-240338bf8eb5");

// Message 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Msg1 {
    pub header: MsgHeader,
}

impl Default for Msg1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Msg1 {
    pub fn new() -> Self {
        Self {
            header: MsgHeader { id: MSG1_ID },
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }
}

#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_msg1_serde() {
        let msg1 = Box::<Msg1>::default();
        println!("test_msg1_serde: msg1: {msg1:?}");
        let ser_msg1 = serde_json::to_string(&msg1).unwrap();
        println!("test_msg1_serde: ser_msg1={ser_msg1}");
        let deser_msg1: Box<Msg1> = serde_json::from_str(&ser_msg1).unwrap();
        println!("test_msg1_serde: deser_msg1={deser_msg1:?}");
        assert_eq!(msg1.header.id, MSG1_ID);
        assert_eq!(msg1.header.id, deser_msg1.header.id);
        println!(
            "test_msg1_serde: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            (*deser_msg1).type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), (*deser_msg1).type_id());
    }
}
