use std::{error::Error, result::Result, str::from_utf8};

use msg_header::{MsgHeader, MsgId};
use serde::{Deserialize, Serialize};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG1_ID_STR: &str = "a88ba7e7-0930-4df6-bb24-240338bf8eb5";
pub const MSG1_ID: MsgId = uuid!("a88ba7e7-0930-4df6-bb24-240338bf8eb5");

// Message 1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn from_serde_json_str(s: &str) -> Result<Self, Box<dyn Error>> {
        if MsgHeader::cmp_str_id_and_serde_json_msg_header(MSG1_ID_STR, s) {
            match serde_json::from_str::<Self>(s) {
                Ok(msg) => Ok(msg),
                Err(why) => Err(format!("Msg1::from_serde_json_str: {why}").into()),
            }
        } else {
            Err(
                format!("Msg1::from_serde_json_str: wrong id in {s}, expecting {MSG1_ID_STR}")
                    .into(),
            )
        }
    }

    pub fn from_serde_json_buf(buf: &[u8]) -> Result<Self, Box<dyn Error>> {
        if let Ok(s) = from_utf8(buf) {
            Self::from_serde_json_str(s)
        } else {
            Err("Msg1::from_serde_json_buf: Not UTF8".into())
        }
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

    #[test]
    fn test_msg1_from_json_str() {
        let msg1 = Box::<Msg1>::default();
        let ser_msg1 = serde_json::to_string(&msg1).unwrap();
        let msg1_from_serde_json_str = Msg1::from_serde_json_str(ser_msg1.as_str()).unwrap();
        assert_eq!(*msg1, msg1_from_serde_json_str);
    }

    #[test]
    fn test_msg1_from_json_buf() {
        let msg1 = Box::<Msg1>::default();
        let ser_msg1 = serde_json::to_string(&msg1).unwrap();
        let msg1_from_serde_json_str = Msg1::from_serde_json_buf(ser_msg1.as_bytes()).unwrap();
        assert_eq!(*msg1, msg1_from_serde_json_str);
    }
}
