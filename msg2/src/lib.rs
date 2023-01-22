use msg_header::MsgHeader;

// Message 2
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg2 {
    pub header: MsgHeader,
}
