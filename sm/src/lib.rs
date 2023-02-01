use msg_header::BoxMsgAny;

// Dispatch a message
pub trait ProcessMsgAny {
    fn process_msg_any(&mut self, msg: BoxMsgAny);
}
