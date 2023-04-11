#![feature(downcast_unchecked)] // Disable if stable
use actor_channel::ActorSender;
use an_id::AnId;
use box_msg_any::BoxMsgAny;
use sender_map_by_instance_id::sender_map_get;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
mod get_msg_id_str_from_buf;
pub use get_msg_id_str_from_buf::{get_msg_id_str_from_buf, FromSerdeJsonBuf, ToSerdeJsonBuf};

// You can use stable but the result will NOT run,
// I've added this to debug the issue with Rust-Analyzer
// reporting a false error for sender_map_insert.
// See:
//  https://users.rust-lang.org/t/ra-reports-errors-but-compile-succeeds/92127
//  https://github.com/rust-lang/rust-analyzer/pull/14475
#[rustversion::stable]
use an_id::anid;
#[rustversion::stable]
use paste::paste;

#[rustversion::stable]
const DEBUG_ANID: AnId = anid!("def26b5a-a462-492a-acdd-85aac7a1f2ac");
#[rustversion::stable]
const SOME_DEBUG_ANID: Option<AnId> = anid!("c4e6ad97-8661-491e-a4fc-a986bbb1cf45");

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub msg_id: AnId,
    pub src_id: AnId,
}

impl MsgHeader {
    pub fn new(msg_id: AnId, src_id: AnId) -> Self {
        //println!("MsgHeader::new");
        Self { msg_id, src_id }
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
    pub fn get_src_id_from_boxed_msg_any(msg_any: &BoxMsgAny) -> &AnId {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg_any.downcast_ref_unchecked() };

        &mh.src_id
    }

    #[rustversion::stable]
    pub fn get_src_id_from_boxed_msg_any(_msg_any: &BoxMsgAny) -> &AnId {
        &SOME_DEBUG_ANID
    }

    pub fn get_src_tx_from_boxed_msg_any(msg_any: &BoxMsgAny) -> Option<ActorSender> {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(msg_any);

        sender_map_get(src_id)
    }

    pub fn simple_display(&self) -> String {
        format!(
            "mh {{ msg_id: {} src_id: {} }}",
            &self.msg_id.to_string()[0..8],
            &self.src_id.to_string()[0..8],
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
    use an_id::{anid, paste};

    use super::*;

    #[test]
    fn test_size() {
        println!("\n");
        let header = MsgHeader::new(AnId::nil(), AnId::nil());
        let size = std::mem::size_of_val(&header);
        println!("test_default: size_of_val(&header)={size}    {{header}}={header}");
        println!("test_default: size_of_val(&header)={size}  {{header:?}}={header:?}");
        println!("test_default: size_of_val(&header)={size} {{header:#?}}={header:#?}");
        assert_eq!(size, 32);
    }

    #[test]
    fn test_new() {
        println!("\n");
        let msg_id = AnId::nil();
        let src_id = anid!("31d1ee24-0dfd-49cd-906a-857aa67e59f4");

        let header = MsgHeader::new(msg_id, src_id);
        println!("test_new:    {{header}}={header}");
        println!("test_new:  {{header:?}}={header:?}");
        println!("test_new: {{header:#?}}={header:#?}");
        assert_eq!(header.msg_id, msg_id);
        assert_eq!(header.src_id, src_id);
    }

    #[test]
    fn test_msg_id_utf8_len() {
        println!("\n");
        let nill_utf8: String = AnId::nil().to_string();
        println!("test_msg_id_utf8_len: nil_utf8={nill_utf8}");
        assert_eq!(MSG_ID_STR_LEN, nill_utf8.len());
    }
}
