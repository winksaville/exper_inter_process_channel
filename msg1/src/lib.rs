use msg_header::{MsgHeader, MsgId};
use serde::{Deserialize, Serialize};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG1_ID_STR: &str = "a88ba7e7-0930-4df6-bb24-240338bf8eb5";
pub const MSG1_ID: MsgId = uuid!("a88ba7e7-0930-4df6-bb24-240338bf8eb5");
pub const MSG1_NAME: &str = "Msg1";

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

// Allow `clippy::uninlined_format_args`  because in msg_macro
// we need to use stringify!($name) which can't be used in a
// format string. Also this is caught by `cargo +nightly clippy`.
#[allow(clippy::uninlined_format_args)]
impl Msg1 {
    pub fn new() -> Self {
        Self {
            header: MsgHeader { id: MSG1_ID },
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }

    pub fn to_serde_json_string(&self) -> std::option::Option<String> {
        match serde_json::to_string(self) {
            Ok(v) => Some(v),
            Err(why) => {
                log::error!("{}.to_serde_json_string: Error {}", MSG1_NAME, why);
                None
            }
        }
    }

    pub fn from_serde_json_str(s: &str) -> std::option::Option<Self> {
        if msg_header::MsgHeader::cmp_str_id_and_serde_json_msg_header(MSG1_ID_STR, s) {
            match serde_json::from_str::<Self>(s) {
                Ok(msg) => Some(msg),
                Err(why) => {
                    log::error!("{}::from_serde_json_str: {why}", MSG1_NAME);
                    None
                }
            }
        } else {
            log::trace!(
                "{}::from_serde_json_str: wrong id in {s}, expecting {}",
                MSG1_NAME,
                MSG1_ID_STR
            );
            None
        }
    }

    pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<Self> {
        if let Ok(s) = std::str::from_utf8(buf) {
            Self::from_serde_json_str(s)
        } else {
            log::error!("{}::from_serde_json_buf: Not UTF8", MSG1_NAME);
            None
        }
    }

    pub fn to_serde_json_buf(&self) -> std::option::Option<Vec<u8>> {
        match serde_json::to_vec(self) {
            Ok(v) => Some(v),
            Err(why) => {
                log::error!("{}.to_serde_json_buf: Error {}", MSG1_NAME, why);
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_msg1_to_from_json_str() {
        let msg1 = Box::<Msg1>::default();
        println!("test_msg1_to_from_json_str: msg1: {msg1:?}");
        let ser_msg1 = msg1.to_serde_json_string().unwrap();
        println!("test_msg1_to_from_json_str: ser_msg1={ser_msg1}");
        let msg1_deser = Msg1::from_serde_json_str(&ser_msg1).unwrap();
        println!("test_msg1_to_from_json_str: msg1_deser={msg1_deser:?}");
        assert_eq!(*msg1, msg1_deser);
        assert_eq!(msg1.header.id, MSG1_ID);
        assert_eq!(msg1.header.id, msg1_deser.header.id);
        println!(
            "test_msg1_to_from_json_str: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            msg1_deser.type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), msg1_deser.type_id());
    }

    #[test]
    fn test_msg1_to_from_json_buf() {
        let msg1 = Box::<Msg1>::default();
        println!("test_msg1_to_from_json_buf: msg1: {msg1:?}");
        let ser_msg1 = msg1.to_serde_json_buf().unwrap();
        println!("test_msg1_to_from_json_buf: ser_msg1={ser_msg1:x?}");
        let msg1_deser = Msg1::from_serde_json_buf(&ser_msg1).unwrap();
        println!("test_msg1_to_from_json_buf: msg1_deser={msg1_deser:?}");
        assert_eq!(*msg1, msg1_deser);
        assert_eq!(msg1.header.id, MSG1_ID);
        assert_eq!(msg1.header.id, msg1_deser.header.id);
        println!(
            "test_msg1_to_from_json_buf: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            msg1_deser.type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), msg1_deser.type_id());
    }
}
