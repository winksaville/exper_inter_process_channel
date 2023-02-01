use msg_header::{BoxMsgAny, MsgHeader, MsgId};
use msg_serde_json::cmp_str_id_and_serde_json_msg_header;
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
    pub fu64: u64,
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
            fu64: 0x123456789ABCDEF1,
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }

    pub fn from_box_msg_any(msg: &BoxMsgAny) -> Option<&Msg1> {
        if let Some(msg1) = msg.downcast_ref::<Self>() {
            Some(msg1)
        } else {
            None
        }
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
        if cmp_str_id_and_serde_json_msg_header(MSG1_ID_STR, s) {
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

    pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<BoxMsgAny> {
        if let Ok(s) = std::str::from_utf8(buf) {
            if let Some(m) = Self::from_serde_json_str(s) {
                Some(Box::new(m))
            } else {
                None
            }
        } else {
            log::error!("{}::from_serde_json_buf: Not UTF8", MSG1_NAME);
            None
        }
    }

    pub fn to_serde_json_buf(boxed_msg_any: BoxMsgAny) -> std::option::Option<Vec<u8>> {
        if let Some(m) = boxed_msg_any.downcast_ref::<Self>() {
            match serde_json::to_vec(m) {
                Ok(v) => Some(v),
                Err(why) => {
                    log::error!("{}.to_serde_json_buf: Error {}", MSG1_NAME, why);
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use msg_serde_json::{FromSerdeJsonBuf, ToSerdeJsonBuf};
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    #[test]
    fn test_msg1_to_from_serde_json_str() {
        let msg1 = Box::<Msg1>::default();
        println!("test_msg1_to_from_serde_json_str: msg1: {msg1:?}");
        let ser_msg1 = Msg1::to_serde_json_string(&msg1).unwrap();
        println!("test_msg1_to_from_serde_json_str: ser_msg1={ser_msg1}");
        let msg1_deser = Msg1::from_serde_json_str(&ser_msg1).unwrap();
        println!("test_msg1_to_from_serde_json_str: msg1_deser={msg1_deser:?}");
        assert_eq!(*msg1, msg1_deser);
        assert_eq!(msg1.header.id, MSG1_ID);
        assert_eq!(msg1.header.id, msg1_deser.header.id);
        println!(
            "test_msg1_to_from_serde_json_str: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            msg1_deser.type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), msg1_deser.type_id());
    }

    #[test]
    fn test_msg1_to_from_serde_json_buf() {
        let msg1 = Msg1::default();
        println!("test_msg1_to_from_serde_json_buf: msg1: {msg1:?}");
        let bma1: BoxMsgAny = Box::new(msg1.clone());
        let ser_msg1 = Msg1::to_serde_json_buf(bma1).unwrap();
        let bma1_deser = Msg1::from_serde_json_buf(&ser_msg1).unwrap();
        let msg1_deser = bma1_deser.downcast_ref::<Msg1>().unwrap();
        println!("test_msg1_to_from_serde_json_buf: msg1_deser={msg1_deser:?}");
        assert_eq!(&msg1, msg1_deser);
        assert_eq!(msg1.header.id, MSG1_ID);
        assert_eq!(msg1.header.id, msg1_deser.header.id);
        println!(
            "test_msg1_to_from_serde_json_buf: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            msg1_deser.type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), msg1_deser.type_id());
    }

    #[test]
    fn test_hash_map_to_from_serde_json_buf() {
        let msg1 = Msg1::default();
        println!("MSG1_ID_STR={MSG1_ID_STR}");
        println!("msg1={msg1:?}");

        let msg1_serde_json_string = msg1.to_serde_json_string().unwrap();
        println!("{msg1_serde_json_string}");

        let msg1_deser = Msg1::from_serde_json_str(&msg1_serde_json_string).unwrap();
        println!("msg1_deser={msg1_deser:?}");
        assert_eq!(msg1_deser, msg1);

        // Use HashMap to serialize
        let ser = Msg1::to_serde_json_buf;
        let mut hm_ser = HashMap::<MsgId, ToSerdeJsonBuf>::new();
        hm_ser.insert(msg1.id(), ser);
        println!("hm_ser.len()={}", hm_ser.len());

        // Use HashMap to deserialize
        let deser = Msg1::from_serde_json_buf;
        let mut hm_deser = HashMap::<MsgId, FromSerdeJsonBuf>::new();
        hm_deser.insert(msg1.id(), deser);
        println!("hm_deser.len()={}", hm_deser.len());

        // Instantiate msg1 as a BoxMsgAny
        let msg1_box_msg_any: BoxMsgAny = Box::new(msg1.clone());

        // Get to_serde funtion and serialize
        let fn_to_serde_json_buf = hm_ser.get(&msg1.id()).unwrap();
        let msg1_buf = (*fn_to_serde_json_buf)(msg1_box_msg_any).unwrap();
        println!("msg1_buf {:p} {msg1_buf:x?}", &*msg1_buf);
        println!("msg1_buf uf8={:?}", std::str::from_utf8(&msg1_buf).unwrap());

        // Get from_serde function and deserialize
        let fn_from_serde_json_buf = hm_deser.get(&msg1.id()).unwrap();
        let msg1_serde_box_msg_any = (*fn_from_serde_json_buf)(&msg1_buf).unwrap();
        println!(
            "msg1_serde_box_msg_any: {:p} {:p} {} {msg1_serde_box_msg_any:?}",
            msg1_serde_box_msg_any,
            &*msg1_serde_box_msg_any,
            std::mem::size_of::<BoxMsgAny>()
        );

        let msg1_serde = if let Some(msg) = Msg1::from_box_msg_any(&msg1_serde_box_msg_any) {
            msg
        } else {
            panic!("msg1_serde was not a Msg1");
        };
        println!(
            "msg1_serde: {:p} {:?} {} {msg1_serde:?}",
            msg1_serde,
            msg1_serde.type_id(),
            std::mem::size_of::<Msg1>()
        );
        assert_eq!(&msg1, msg1_serde);
    }
}
