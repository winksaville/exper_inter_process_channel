use std::any::Any;

// Messages are things that implement trait std::any::Any
// which is most anything
pub type MsgAny = dyn Any;

// This type alias is generic and apparently can't be exported
// but Message can, oh well.
//pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

// Dispatch a message
pub trait ProcessMsgAny {
    fn process_msg_any(&mut self, msg: Box<MsgAny>);
}

//pub mod msg1;
//pub mod msg2;
//pub mod msg_header;
//pub mod sm_channel_to_network;
