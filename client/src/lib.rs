use echo_reply::{EchoReply, ECHO_REPLY_ID};
use echo_req::{EchoReq, ECHO_REQ_ID};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use msg_header::BoxMsgAny;
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
    pub fn new(name: &str, initial_state: ProcessMsgFn<Self>) -> Self {
        let mut this = Self {
            name: name.to_owned(),
            current_state: initial_state,
            state_info_hash: StateInfoMap::<Self>::new(),
        };

        this.add_state(Client::state0, "state0");
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

    pub fn state0(&mut self, msg: BoxMsgAny) {
        if let Some(m) = msg.downcast_ref::<EchoReply>() {
            assert_eq!(m.header.id, ECHO_REPLY_ID);
            println!("{}:State0: {m:?}", self.name);
        } else if let Some(m) = msg.downcast_ref::<EchoReq>() {
            assert_eq!(m.header.id, ECHO_REQ_ID);
            println!("{}:State0: {m:?}", self.name);
        } else {
            let msg_id = msg_header::get_msg_id_from_boxed_msg_any(&msg);
            println!("{}:State0: Unknown msg={msg:?} {msg_id:?}", self.name);
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
    use msg1::Msg1; //, MSG1_ID};

    #[test]
    fn test_1() {
        let mut client = Client::new("client", Client::state0);
        println!("test_1: client={client:?}");

        let msg1 = Msg1::default();
        println!("test_1: msg1={msg1:?}");

        client.process_msg_any(Box::new(msg1));
    }
}
