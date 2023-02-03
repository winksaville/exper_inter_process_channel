use echo_reply::{EchoReply, ECHO_REPLY_ID};
use echo_req::{EchoReq, ECHO_REQ_ID};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    sync::mpsc::Sender,
};

use msg_header::{BoxMsgAny, MsgHeader};
use sm::ProcessMsgAny;

type ProcessMsgFn<SM> = fn(&mut SM, BoxMsgAny);

// State information
#[derive(Debug)]
pub struct StateInfo {
    pub name: String,
}

// HashMap that maps address of a ProcessMsgFn to StateInfo
type StateInfoMap<SM> = HashMap<*const ProcessMsgFn<SM>, StateInfo>;

// State machine for channel to network
pub struct Client {
    pub name: String,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_tx: Sender<BoxMsgAny>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fn_ptr = self.current_state as *const ProcessMsgFn<Self>;
        let fn_ptr_string = format!("{fn_ptr:p}");
        let state_name = if let Some(n) = self.state_info_hash.get(&fn_ptr) {
            // State does have a name
            n.name.as_str()
        } else {
            // State does NOT have a name, use address
            fn_ptr_string.as_str()
        };

        write!(
            f,
            "{} {{ name: {}, state_info_hash: {:?}; current_state: {state_name} }}",
            self.name, self.name, self.state_info_hash
        )
    }
}

impl Client {
    pub fn new(
        name: &str,
        initial_state: ProcessMsgFn<Self>,
        partner_tx: Sender<BoxMsgAny>,
    ) -> Self {
        let mut this = Self {
            name: name.to_owned(),
            current_state: initial_state,
            state_info_hash: StateInfoMap::<Self>::new(),
            partner_tx,
        };

        this.add_state(Self::state0, "state0");
        this
    }

    pub fn add_state(&mut self, state: ProcessMsgFn<Self>, name: &str) {
        let s = StateInfo {
            name: name.to_owned(),
        };
        let k = state as *const ProcessMsgFn<Self>;
        self.state_info_hash.insert(k, s);
    }

    #[allow(unused)]
    fn transition(&mut self, dest: ProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    pub fn state0(&mut self, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReply>() {
            assert_eq!(msg.header.id, ECHO_REPLY_ID);
            println!("{}:State0: {msg:?}", self.name);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            let reply_msg = Box::new(EchoReply::from_echo_req(msg));
            println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
            self.partner_tx.send(reply_msg).unwrap();
        } else {
            let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
            println!(
                "{}:State0: Unknown msg_any={msg_any:?} {msg_id:?}",
                self.name
            );
        }
    }
}

impl ProcessMsgAny for Client {
    fn process_msg_any(&mut self, msg: BoxMsgAny) {
        (self.current_state)(self, msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::mpsc::channel;

    #[test]
    fn test_1() {
        let (tx, rx) = channel();

        let mut client = Client::new("client", Client::state0, tx);
        println!("test_1: client={client:?}");

        let echo_req = EchoReq::new("a message", 1);
        println!("test_1: echo_req={echo_req:?}");

        client.process_msg_any(Box::new(echo_req));
        let reply_msg_any = rx.recv().unwrap();
        let received_msg = reply_msg_any.downcast_ref::<EchoReply>().unwrap();
        println!("test_1: received msg={received_msg:?}");
    }
}
