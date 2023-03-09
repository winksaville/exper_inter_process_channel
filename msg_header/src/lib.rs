#![feature(downcast_unchecked)]
//use std::hash::{Hash, Hasher};
use std::fmt;

use an_id::AnId;
use serde::{Deserialize, Serialize};

// Messages are things that implement trait std::any::Any
// which is most anything
pub type BoxMsgAny = Box<dyn std::any::Any + Send>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MsgId(pub AnId);

// This implicilty defines to_string, as advised by clippy
// https://rust-lang.github.io/rust-clippy/master/index.html#inherent_to_string
impl fmt::Display for MsgId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}

impl Default for MsgHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgHeader {
    pub fn new() -> Self {
        Self {
            id: MsgId(AnId::new()),
        }
    }

    pub fn get_msg_id_from_boxed_msg_any(msg: &BoxMsgAny) -> &MsgId {
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let msg_id: &MsgId = unsafe { msg.downcast_ref_unchecked() };

        msg_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id() {
        let msg_id: MsgId = MsgId(AnId::new());

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
