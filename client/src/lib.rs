use echo_complete::EchoComplete;
use echo_reply::{EchoReply, ECHO_REPLY_ID};
use echo_req::{EchoReq, ECHO_REQ_ID};
use echo_start::{EchoStart, ECHO_START_ID};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    sync::mpsc::Sender,
};

use msg_header::{BoxMsgAny, MsgHeader};
use sm::ProcessMsgAny;

type ProcessMsgFn<SM> = fn(&mut SM, Option<&Sender<BoxMsgAny>>, BoxMsgAny);

// State information
#[derive(Debug)]
pub struct StateInfo {
    pub name: String,
}

// HashMap that maps address of a ProcessMsgFn to StateInfo
type StateInfoMap<SM> = HashMap<*const ProcessMsgFn<SM>, StateInfo>;

/// Client which supports being controlled and a partner
/// to pinging.
///
/// After instantiating the Controller issues an EchoStart with a ping_count.
/// The Client will then ping the partner with an EchoReq and expects
/// the partner to respond with an EchoReply. After pinging the expected
/// number of times the Client will repspond to the Controller with
/// EchoDone.
///
/// Errors and not handled gracefully, this is just demo.
pub struct Client {
    pub name: String,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_tx: Sender<BoxMsgAny>, // TODO: Allow each message to have available a "reply Sender"
    pub controller_tx: Sender<BoxMsgAny>, // TODO: Allow each message to have available a "reply Sender"
    pub ping_count: u64,
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
            "{} {{ name: {}, state_info_hash: {:?}; current_state: {state_name}; ping_count: {}; }}",
            self.name, self.name, self.state_info_hash, self.ping_count,
        )
    }
}

impl Client {
    pub fn new(
        name: &str,
        initial_state: ProcessMsgFn<Self>,
        partner_tx: Sender<BoxMsgAny>,
        controller_tx: Sender<BoxMsgAny>,
    ) -> Self {
        let mut this = Self {
            name: name.to_owned(),
            current_state: initial_state,
            state_info_hash: StateInfoMap::<Self>::new(),
            partner_tx,
            controller_tx,
            ping_count: 0,
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

    fn send_echo_req_or_complete(&self, reply_tx: Option<&Sender<BoxMsgAny>>, counter: u64) {
        println!(
            "{}:send_echo_req_or_complete:+ counter={counter} ping_count={} * 2 = {}",
            self.name,
            self.ping_count,
            self.ping_count * 2
        );
        if counter <= self.ping_count {
            let req_msg = Box::new(EchoReq::new(counter));
            println!(
                "{}:send_echo_req_or_complete:- to partner_tx msg={req_msg:?}",
                self.name
            );
            if let Some(tx) = reply_tx {
                tx.send(req_msg).unwrap();
            } else {
                self.partner_tx.send(req_msg).unwrap();
            }
        } else {
            println!(
                "{}:send_echo_req_or_complete:- send Complete to controller_tx",
                self.name
            );
            self.controller_tx
                .send(Box::new(EchoComplete::new()))
                .unwrap();
        }
    }

    pub fn state0(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReply>() {
            assert_eq!(msg.header.id, ECHO_REPLY_ID);
            println!("{}:State0: {msg:?}", self.name);
            self.send_echo_req_or_complete(reply_tx, msg.counter + 1);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            let reply_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
            if let Some(tx) = reply_tx {
                tx.send(reply_msg).unwrap();
            } else {
                self.partner_tx.send(reply_msg).unwrap();
            }
        } else if let Some(msg) = msg_any.downcast_ref::<EchoStart>() {
            assert_eq!(msg.header.id, ECHO_START_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            self.ping_count = msg.ping_count;
            self.send_echo_req_or_complete(reply_tx, 1);
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
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny) {
        (self.current_state)(self, reply_tx, msg);
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::*;

    use std::sync::mpsc::channel;

    // Simple test with ping_count 0, there be just
    // a EchoStart then EchoComplete.
    #[test]
    fn test_ping_count_0() {
        // For simple testing this test code will act as the partner and controller
        let (tx, rx) = channel();

        // Using one channel for both partner and controller
        let mut client = Client::new("client", Client::state0, tx.clone(), tx.clone());
        println!("test_1: client={client:?}");

        let start_msg = Box::new(EchoStart::new(0));
        client.process_msg_any(None, start_msg);
        let reply_msg_any = rx.recv().unwrap();
        let received_msg = reply_msg_any.downcast_ref::<EchoComplete>().unwrap();
        println!("test_1: received msg={received_msg:?}");
        assert_eq!(received_msg.header.id, echo_complete::ECHO_COMPLETE_ID);

        drop(tx);
    }

    // Test various ping_counts
    #[test]
    fn test_ping_counts() {
        // For simple testing this test code will act as the partner and controller
        let (tx, rx) = channel();

        // Using one channel for both partner and controller
        let mut client = Client::new("client", Client::state0, tx.clone(), tx.clone());
        println!("test_ping_counts: client={client:?}");

        let reply_tx = tx.clone();
        for ping_count in [0, 1, 5] {
            // Controller sends start message
            println!("\ntest_ping_counts: ping_count={ping_count}");
            let start_msg = Box::new(EchoStart::new(ping_count));
            reply_tx.send(start_msg).unwrap();

            // Client receives Start msg
            let start_msg_any = rx.recv().unwrap();
            client.process_msg_any(Some(&reply_tx), start_msg_any);

            for _ in 0..ping_count {
                // Server receives request message
                let req_msg_any = rx.recv().unwrap();
                let req_msg = req_msg_any.downcast_ref::<EchoReq>().unwrap();
                println!("test_ping_counts: received req_msg={req_msg:?}");

                // Server creates and sends reply message
                let reply_msg = Box::new(EchoReply::new(
                    Utc::now().timestamp_nanos(),
                    req_msg.counter,
                ));
                reply_tx.send(reply_msg).unwrap();

                // Client receives and processes reply message
                let reply_msg_any = rx.recv().unwrap();
                client.process_msg_any(Some(&reply_tx), reply_msg_any);
            }

            // Controller receives Complete msg
            let complete_msg_any = rx.recv().unwrap();
            let complete_msg = complete_msg_any.downcast_ref::<EchoComplete>().unwrap();
            println!("test_ping_counts: received complete msg={complete_msg:?}");
            assert_eq!(complete_msg.header.id, echo_complete::ECHO_COMPLETE_ID);
        }

        drop(tx);
    }
}
