use msg1::Msg1;
use msg2::Msg2;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use sm::{MsgAny, ProcessMsgAny};

type ProcessMsgFn<SM> = fn(&mut SM, Box<MsgAny>);

// State information
#[derive(Debug)]
pub struct StateInfo {
    pub name: String,
}

// HashMap that maps address of a ProcessMsgFn to StateInfo
type StateInfoMap<SM> = HashMap<*const ProcessMsgFn<SM>, StateInfo>;

// State machine for channel to network
pub struct SmNetworkToChannel {
    pub name: String,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
}

impl Debug for SmNetworkToChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let k = self.current_state as *const ProcessMsgFn<Self>;
        let kstring = format!("{k:p}");
        let name = if let Some(n) = self.state_info_hash.get(&k) {
            n.name.as_str()
        } else {
            kstring.as_str()
        };

        write!(
            f,
            "SmNetworkToChannel {{ name: {}, state_info_hash: {:?}; current_state: {name} }}",
            self.name, self.state_info_hash
        )
    }
}

impl SmNetworkToChannel {
    pub fn new(name: &str, initial_state: ProcessMsgFn<Self>) -> Self {
        Self {
            name: name.to_owned(),
            current_state: initial_state,
            state_info_hash: StateInfoMap::<Self>::new(),
        }
    }

    pub fn add_state(&mut self, state: ProcessMsgFn<Self>, name: &str) {
        let s = StateInfo {
            name: name.to_owned(),
        };
        let k = state as *const ProcessMsgFn<Self>;
        self.state_info_hash.insert(k, s);
    }

    fn transition(&mut self, dest: ProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    pub fn state0(&mut self, msg: Box<MsgAny>) {
        if let Some(m) = msg.downcast_ref::<Msg1>() {
            assert_eq!(m.header.id, 1);
            println!("State0: {m:?}");
        } else if let Some(m) = msg.downcast_ref::<Msg2>() {
            assert_eq!(m.header.id, 2);
            println!("State0: {m:?}");
        } else {
            println!("State0: Unknown msg={msg:?}");
        }

        self.transition(SmNetworkToChannel::state1);
    }

    pub fn state1(&mut self, msg: Box<MsgAny>) {
        if let Some(m) = msg.downcast_ref::<Msg1>() {
            assert_eq!(m.header.id, 1);
            println!("State1: {m:?}");
        } else if let Some(m) = msg.downcast_ref::<Msg2>() {
            assert_eq!(m.header.id, 2);
            println!("State1: {m:?}");
        } else {
            println!("State1: Unknown msg={msg:?}");
        }

        self.transition(SmNetworkToChannel::state0);
    }
}

impl ProcessMsgAny for SmNetworkToChannel {
    fn process_msg_any(&mut self, msg: Box<MsgAny>) {
        (self.current_state)(self, msg);
    }
}
