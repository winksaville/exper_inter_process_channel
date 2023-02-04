use std::sync::mpsc::Sender;

use msg_header::BoxMsgAny;

// Dispatch a message
pub trait ProcessMsgAny {
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny);
}
