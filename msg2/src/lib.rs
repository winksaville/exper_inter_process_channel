use std::str::from_utf8;

use msg_header::{MsgHeader, MsgId};
use serde::{Deserialize, Serialize};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG2_ID_STR: &str = "4029b3c4-f380-488a-8560-8320cc8fb76e";
pub const MSG2_ID: MsgId = uuid!("4029b3c4-f380-488a-8560-8320cc8fb76e");

// Message 2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct Msg2 {
    pub header: MsgHeader,
}

impl Default for Msg2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Msg2 {
    pub fn new() -> Self {
        Self {
            header: MsgHeader { id: MSG2_ID },
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }

    pub fn from_serde_json_str(s: &str) -> std::result::Result<Self, &'static str> {
        if MsgHeader::cmp_str_id_and_serde_json_msg_header(MSG2_ID_STR, s) {
            match serde_json::from_str::<Self>(s) {
                Ok(msg) => Ok(msg),
                //Err(why) => Err(format!("{why}").into()),
                Err(_) => Err("from_serde_json_buf: serde_json::from_str::<Msg2>() failed"),
            }
        } else {
            //Err(format!("from_serde_json_buf: wrong id string is not {MSG2_ID_STR}").into())
            Err("from_serde_json_buf: wrong id string")
        }
    }

    pub fn from_serde_json_buf(buf: &[u8]) -> std::result::Result<Self, &'static str> {
        if let Ok(s) = from_utf8(buf) {
            Self::from_serde_json_str(s)
        } else {
            Err("from_serde_json_buf: Not UTF8")
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_msg2_serde() {
        let msg2 = Box::<Msg2>::default();
        println!("test_msg2_serde: msg2: {msg2:?}");
        let ser_msg2 = serde_json::to_string(&msg2).unwrap();
        println!("test_msg2_serde: ser_msg2={ser_msg2}");
        let deser_msg2: Box<Msg2> = serde_json::from_str(&ser_msg2).unwrap();
        println!("test_msg2_serde: deser_msg2={deser_msg2:?}");
        assert_eq!(msg2.header.id, MSG2_ID);
        assert_eq!(msg2.header.id, deser_msg2.header.id);
        println!(
            "test_msg2_serde: TypeId::of::<Msg2>()={:?} msg2.type_id()={:?}",
            TypeId::of::<Msg2>(),
            (*deser_msg2).type_id()
        );
        assert_eq!(TypeId::of::<Msg2>(), (*deser_msg2).type_id());
    }

    #[test]
    fn test_msg2_from_json_str() {
        let msg2 = Box::<Msg2>::default();
        let ser_msg2 = serde_json::to_string(&msg2).unwrap();
        let msg2_from_serde_json_str = Msg2::from_serde_json_str(ser_msg2.as_str()).unwrap();
        assert_eq!(*msg2, msg2_from_serde_json_str);
    }

    #[test]
    fn test_msg2_from_json_buf() {
        let msg2 = Box::<Msg2>::default();
        let ser_msg2 = serde_json::to_string(&msg2).unwrap();
        let msg2_from_serde_json_str = Msg2::from_serde_json_buf(ser_msg2.as_bytes()).unwrap();
        assert_eq!(*msg2, msg2_from_serde_json_str);
    }
}
