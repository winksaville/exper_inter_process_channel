#![feature(downcast_unchecked)]

use std::fmt::{Debug, Display};
use an_id::AnId;
use serde::{Deserialize, Serialize};

// Messages are things that implement trait std::any::Any
// which is most anything
pub type BoxMsgAny = Box<dyn std::any::Any + Send>;

pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub msg_id: AnId,
    pub src_id: Option<AnId>,
}

impl Default for MsgHeader {
    fn default() -> Self {
        println!("MsgHeader::Default::default");
        MsgHeader::default()
    }
}

impl MsgHeader {
    pub fn default() -> Self {
        println!("MsgHeader::default");
        Self::new(AnId::new(), None)
    }

    pub fn new(msg_id: AnId, src_id: Option<AnId>) -> Self {
        println!("MsgHeader::new");
        Self { msg_id, src_id }
    }

    pub fn new_msg_id_only(msg_id: AnId) -> Self {
        println!("MsgHeader::new_msg_id_only");
        Self::new(msg_id, None)
    }

    pub fn get_msg_id_from_boxed_msg_any(msg: &BoxMsgAny) -> &AnId {
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let msg_id: &AnId = unsafe { msg.downcast_ref_unchecked() };

        msg_id
    }

    pub fn simple_display(&self) -> String {
        let s: String = if let Some(sid) = self.src_id { sid.to_string()[0..8].to_string() } else { "None".to_string() };
        format!("mh {{ msg_id: {} src_id: {s} }}", &self.msg_id.to_string()[0..8])
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
