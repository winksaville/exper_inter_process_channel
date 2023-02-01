pub use paste::paste;

#[macro_export]
macro_rules! msg_macro {
    ($name:ident, $id_str:literal) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : msg_header::MsgId = uuid::uuid!($id_str);
        }

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
        #[repr(C)]
        pub struct $name {
            pub header: msg_header::MsgHeader,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        #[allow(unused)]
        impl $name {
            pub fn new() -> Self {
                Self {
                    header: msg_header::MsgHeader {
                        id: uuid::uuid!($id_str),
                    },
                }
            }

            pub fn id(&self) -> msg_header::MsgId {
                self.header.id
            }

            pub fn from_box_msg_any(msg: &msg_header::BoxMsgAny) -> Option<&$name> {
                if let Some(m) = msg.downcast_ref::<$name>() {
                    Some(m)
                } else {
                    None
                }
            }

            pub fn to_serde_json_string(&self) -> std::option::Option<String> {
                match serde_json::to_string(self) {
                    Ok(v) => Some(v),
                    Err(why) => {
                        log::error!("{}.to_serde_json_string: Error {}", stringify!($name), why);
                        None
                    }
                }
            }

            pub fn from_serde_json_str(s: &str) -> std::option::Option<Self> {
                if msg_serde_json::cmp_str_id_and_serde_json_msg_header($id_str, s) {
                    match serde_json::from_str::<Self>(s) {
                        Ok(msg) => Some(msg),
                        Err(why) => {
                            log::error!("{}::from_serde_json_str: {why}", stringify!($name));
                            None
                        }
                    }
                } else {
                    log::trace!(
                        "{}::from_serde_json_str: wrong id in {s}, expecting {}",
                        stringify!($name),
                        $id_str
                    );
                    None
                }
            }

            pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<msg_header::BoxMsgAny> {
                if let Ok(s) = std::str::from_utf8(buf) {
                    if let Some(m) = Self::from_serde_json_str(s) {
                        Some(Box::new(m))
                    } else {
                        None
                    }
                } else {
                    log::error!("{}::from_serde_json_buf: Not UTF8", stringify!($name));
                    None
                }
            }

            pub fn to_serde_json_buf(
                boxed_msg_any: msg_header::BoxMsgAny,
            ) -> std::option::Option<Vec<u8>> {
                if let Some(m) = boxed_msg_any.downcast_ref::<Self>() {
                    match serde_json::to_vec(m) {
                        Ok(v) => Some(v),
                        Err(why) => {
                            log::error!("{}.to_serde_json_buf: Error {}", stringify!($name), why);
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};

    use msg_header::BoxMsgAny;

    use super::*;

    // From: https://www.uuidgenerator.net/version4
    msg_macro!(MsgA, "d122e9aa-0a69-4654-8e41-e2813bc40272");

    #[test]
    fn test_msg_a_serde() {
        let msg_a = Box::<MsgA>::default();
        println!("test_msg_a_serde: msg_a: {msg_a:?}");
        let ser_msg_a = msg_a.to_serde_json_string().unwrap();
        println!("test_msg_a_serde: ser_msg_a={ser_msg_a}");
        let deser_msg_a: MsgA = MsgA::from_serde_json_str(&ser_msg_a).unwrap();
        println!("test_msg_a_serde: deser_msg_a={deser_msg_a:?}");
        assert_eq!(msg_a.header.id, MSG_A_ID);
        assert_eq!(msg_a.header.id, deser_msg_a.header.id);
        println!(
            "test_msg_a_serde: TypeId::of::<MsgA>()={:?} msg_a.type_id()={:?}",
            TypeId::of::<MsgA>(),
            deser_msg_a.type_id()
        );
        assert_eq!(TypeId::of::<MsgA>(), deser_msg_a.type_id());
    }

    #[test]
    fn test_msg_a_from_json_str() {
        let msg_a = Box::<MsgA>::default();
        let ser_msg_a = msg_a.to_serde_json_string().unwrap();
        let msg_a_from_serde_json_str = MsgA::from_serde_json_str(ser_msg_a.as_str()).unwrap();
        assert_eq!(*msg_a, msg_a_from_serde_json_str);
    }

    #[test]
    fn test_msg_a_to_from_serde_json_buf() {
        let msg_a = Box::<MsgA>::default();
        let msg_a_any_1: BoxMsgAny = msg_a.clone();
        let msg_a_vec = MsgA::to_serde_json_buf(msg_a_any_1).unwrap();
        let msg_a_any_2 = MsgA::from_serde_json_buf(&msg_a_vec).unwrap();
        let msg_a_deser = MsgA::from_box_msg_any(&msg_a_any_2).unwrap();
        assert_eq!(&*msg_a, msg_a_deser);
    }
}
