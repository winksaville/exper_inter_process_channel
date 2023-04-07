//#![feature(downcast_unchecked)] // Disable if stable
use actor_channel::ActorSender;
use an_id::AnId;
use box_msg_any::BoxMsgAny;
use sender_map_by_instance_id::sender_map_get;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
mod get_msg_id_str_from_buf;
pub use get_msg_id_str_from_buf::{get_msg_id_str_from_buf, FromSerdeJsonBuf, ToSerdeJsonBuf};

// You can use stable but the result will no run,
// I've added this to debug the issue with Rust-Analyzer
// reporting a false error for sender_map_insert.
#[rustversion::stable]
use paste::paste;
#[rustversion::stable]
use an_id::anid;

#[rustversion::stable]
const DEBUG_ANID: AnId = anid!("def26b5a-a462-492a-acdd-85aac7a1f2ac");
#[rustversion::stable]
const SOME_DEBUG_ANID: Option<AnId> = Some(anid!("c4e6ad97-8661-491e-a4fc-a986bbb1cf45"));

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub msg_id: AnId,
    pub src_id: Option<AnId>, // TODO: Remove optional, it could be AnId::null() instead
}

impl Default for MsgHeader {
    fn default() -> Self {
        println!("MsgHeader::Default::default");
        Self::new(AnId::new(), None)
    }
}

impl MsgHeader {
    pub fn new(msg_id: AnId, src_id: Option<AnId>) -> Self {
        //println!("MsgHeader::new");
        Self { msg_id, src_id }
    }

    // TODO: Consider remove new_msg_id_only if src_id becomes non-optional
    #[deprecated]
    pub fn new_msg_id_only(msg_id: AnId) -> Self {
        //println!("MsgHeader::new_msg_id_only");
        Self::new(msg_id, None)
    }

    #[rustversion::nightly]
    pub fn get_msg_id_from_boxed_msg_any(msg: &BoxMsgAny) -> &AnId {
        // TODO: Consider validating that the msg_header or AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg.downcast_ref_unchecked() };
        &mh.msg_id
    }

    #[rustversion::stable]
    pub fn get_msg_id_from_boxed_msg_any(_msg: &BoxMsgAny) -> &AnId {
        &DEBUG_ANID
    }

    #[rustversion::nightly]
    pub fn get_src_id_from_boxed_msg_any(msg_any: &BoxMsgAny) -> &Option<AnId> {
        // TODO: Consider validating that the msg_header or AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg_any.downcast_ref_unchecked() };

        &mh.src_id
    }

    #[rustversion::stable]
    pub fn get_src_id_from_boxed_msg_any(_msg_any: &BoxMsgAny) -> &Option<AnId> {
        &SOME_DEBUG_ANID
    }

    pub fn get_src_tx_from_boxed_msg_any(msg_any: &BoxMsgAny) -> Option<ActorSender> {
        // TODO: Consider validating that the msg_header or AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let option_src_id = MsgHeader::get_src_id_from_boxed_msg_any(msg_any);

        let sender = match option_src_id {
            Some(src_id) => {
                if let Some(sender) = sender_map_get(src_id) {
                    Some(sender)
                } else {
                    let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(msg_any);
                    panic!( "get_src_id_from_boxed_msg_any: BUG; msg_any has msg_id={msg_id} and src_id={src_id:?} but not in sender_map");
                }
                //let (tx, _) = crossbeam_channel::unbounded();
                //Some(ActorSender::new("xx", tx))
            }
            None => {
                let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(msg_any);
                panic!("get_src_id_from_boxed_msg_any: There is no src_id in msg_any header.msg_id={msg_id:?}",);
            }
        };

        sender
    }

    pub fn simple_display(&self) -> String {
        let s: String = if let Some(sid) = self.src_id {
            sid.to_string()[0..8].to_string()
        } else {
            "None".to_string()
        };
        format!(
            "mh {{ msg_id: {} src_id: {s} }}",
            &self.msg_id.to_string()[0..8]
        )
    }
}

impl Debug for MsgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("MsgHeader")
                .field("id", &self.msg_id)
                .field("source_id", &self.src_id)
                .finish()
        } else {
            write!(f, "{}", self.simple_display())
        }
    }
}

impl Display for MsgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        println!("\n");
        let header: MsgHeader = Default::default();
        let size = std::mem::size_of_val(&header);
        println!("test_default: size_of_val(&header)={size}    {{header}}={header}");
        println!("test_default: size_of_val(&header)={size}  {{header:?}}={header:?}");
        println!("test_default: size_of_val(&header)={size} {{header:#?}}={header:#?}");
        assert_eq!(size, 33);
    }

    #[test]
    fn test_default() {
        println!("\n");
        let header: MsgHeader = Default::default();
        println!("test_default:    {{header}}={header}");
        println!("test_default:  {{header:?}}={header:?}");
        println!("test_default: {{header:#?}}={header:#?}");
        assert_ne!(header.msg_id, AnId::nil());
        assert_eq!(header.src_id, None);
    }

    #[test]
    fn test_msgheader_default() {
        println!("\n");
        let header = MsgHeader::default();
        println!("test_msgheader_default:    {{header}}={header}");
        println!("test_msgheader_default:  {{header:?}}={header:?}");
        println!("test_msgheader_default: {{header:#?}}={header:#?}");
        assert_ne!(header.msg_id, AnId::nil());
        assert_eq!(header.src_id, None);
    }

    #[test]
    fn test_new() {
        println!("\n");
        let msg_id = AnId::new();
        let src_id = AnId::new();

        let header = MsgHeader::new(msg_id, Some(src_id));
        println!("test_new:    {{header}}={header}");
        println!("test_new:  {{header:?}}={header:?}");
        println!("test_new: {{header:#?}}={header:#?}");
        assert_eq!(header.msg_id, msg_id);
        assert_eq!(header.src_id, Some(src_id));
    }

    #[test]
    fn test_new_msg_id_only() {
        println!("\n");
        let msg_id = AnId::new();

        #[allow(deprecated)]
        let header = MsgHeader::new_msg_id_only(msg_id);
        println!("test_new:    {{header}}={header}");
        println!("test_new:  {{header:?}}={header:?}");
        println!("test_new: {{header:#?}}={header:#?}");
        assert_eq!(header.msg_id, msg_id);
        assert_eq!(header.src_id, None);
    }

    #[test]
    fn test_msg_id_utf8_len() {
        println!("\n");
        let nill_utf8: String = AnId::nil().to_string();
        println!("test_msg_id_utf8_len: nil_utf8={nill_utf8}");
        assert_eq!(MSG_ID_STR_LEN, nill_utf8.len());
    }
}
