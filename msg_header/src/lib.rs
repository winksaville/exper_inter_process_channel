#![feature(downcast_unchecked)]

use an_id::AnId;
use serde::{Deserialize, Serialize};

// Messages are things that implement trait std::any::Any
// which is most anything
pub type BoxMsgAny = Box<dyn std::any::Any + Send>;

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub id: AnId,
}

impl Default for MsgHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgHeader {
    pub fn new() -> Self {
        Self { id: AnId::new() }
    }

    pub fn get_msg_id_from_boxed_msg_any(msg: &BoxMsgAny) -> &AnId {
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let msg_id: &AnId = unsafe { msg.downcast_ref_unchecked() };

        msg_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id() {
        let msg_id = AnId::new();

        let header = MsgHeader { id: msg_id };
        println!("test_id: header={header:?}");
        assert_eq!(msg_id, header.id);
    }

    #[test]
    fn test_msg_id_utf8_len() {
        let nill_utf8: String = AnId::nil().to_string();
        println!("test_msg_id_utf8_len: nil_utf8={nill_utf8}");
        assert_eq!(MSG_ID_STR_LEN, nill_utf8.len());
    }
}
