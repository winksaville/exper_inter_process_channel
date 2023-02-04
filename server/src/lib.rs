use echo_reply::EchoReply;
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
pub struct Server {
    pub name: String,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_tx: Sender<BoxMsgAny>,
}

impl Debug for Server {
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

impl Server {
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
        if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            //println!("{}:State0: msg={msg:?}", self.name);
            let reply_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            //println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
            self.partner_tx.send(reply_msg).unwrap();
        //} else {
        //    let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
        //    println!(
        //        "{}:State0: Unknown msg_any={msg_any:?} {msg_id:?}",
        //        self.name
        //    );
        }
    }
}

impl ProcessMsgAny for Server {
    fn process_msg_any(&mut self, msg: BoxMsgAny) {
        (self.current_state)(self, msg);
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::*;

    use std::sync::mpsc::channel;

    #[test]
    fn test_1() {
        let (tx, rx) = channel();

        let mut server = Server::new("server", Server::state0, tx);
        println!("test_1: server={server:?}");

        let first_now_ns = Utc::now().timestamp_nanos();
        let second_now_ns = Utc::now().timestamp_nanos();
        let third_now_ns = Utc::now().timestamp_nanos();

        let now_ns = Utc::now().timestamp_nanos();

        let echo_req = EchoReq::new(1);
        server.process_msg_any(Box::new(echo_req.clone()));
        let reply_msg_any = rx.recv().unwrap();
        let received_msg = reply_msg_any.downcast_ref::<EchoReply>().unwrap();

        let last_ns = Utc::now().timestamp_nanos();

        assert!(echo_req.req_timestamp_ns >= now_ns);
        assert_eq!(received_msg.req_timestamp_ns, echo_req.req_timestamp_ns);
        assert!(received_msg.req_timestamp_ns >= now_ns);
        assert!(last_ns >= received_msg.req_timestamp_ns);
        
        println!("test_1: echo_req={echo_req:?}");
        println!("test_1: received msg = {received_msg:?}");
        println!();
        println!("test_1:          second_now_ns - first_now_ns = {:6}ns", second_now_ns - first_now_ns);
        println!("test_1:          third_now_ns - second_now_ns = {:6}ns", third_now_ns - second_now_ns);
        println!("test_1:                 now_ns - third_now_ns = {:6}ns", now_ns - third_now_ns);
        println!("test_1:             req_timestamp_ns - now_ns = {:6}ns", received_msg.req_timestamp_ns - now_ns);
        println!("test_1: reply_timestamp_ns - req_timestamp_ns = {:6}ns", received_msg.reply_timestamp_ns - received_msg.req_timestamp_ns);
        println!("test_1:          last_ns - reply_timestamp_ns = {:6}ns", last_ns - received_msg.reply_timestamp_ns);
        println!("test_1:                RTT = last_ns - now_ns = {:6}ns", last_ns - now_ns);

    }
}
