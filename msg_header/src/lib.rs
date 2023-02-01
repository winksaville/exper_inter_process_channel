//use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Messages are things that implement trait std::any::Any
// which is most anything
pub type BoxMsgAny = Box<dyn std::any::Any + Send>;

pub type MsgId = Uuid;

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::uuid;

    const AN_ID: MsgId = uuid!("3ab7c2f7-6445-4529-a675-5e3246217452");

    #[test]
    fn test_id() {
        let header = MsgHeader { id: AN_ID };
        println!("test_id: header={header:?}");
        assert_eq!(AN_ID, header.id);
    }

    #[test]
    fn test_msg_id_utf8_len() {
        let nill_utf8: String = Uuid::nil().to_string();
        println!("test_msg_id_utf8_len: nil_utf8={nill_utf8}");
        assert_eq!(MSG_ID_STR_LEN, nill_utf8.len());
    }
}
