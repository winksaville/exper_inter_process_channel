use msg_header::MsgHeader;

// Message 1
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg1 {
    pub header: MsgHeader,
}
