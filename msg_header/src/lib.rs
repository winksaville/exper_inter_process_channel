use uuid::Uuid;

pub type MsgId = Uuid;

// Message Header
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}
