pub type Id = u16;

// Message Header
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MsgHeader {
    pub id: Id,
}
