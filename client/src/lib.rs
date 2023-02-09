use actor::{Actor, ActorId, ActorInstanceId};
use echo_complete::EchoComplete;
use echo_reply::{EchoReply, ECHO_REPLY_ID};
use echo_req::{EchoReq, ECHO_REQ_ID};
use echo_requestee_protocol::echo_requestee_protocol;
use echo_requester_protocol::echo_requester_protocol;
use echo_start::{EchoStart, ECHO_START_ID};
use echo_start_complete_protocol::echo_start_complete_protocol;
use protocol::{Protocol, ProtocolId};
use protocol_set::{ProtocolSet, ProtocolSetId};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    sync::mpsc::Sender,
};
use uuid::uuid;

use msg_header::{BoxMsgAny, MsgHeader};

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
    pub actor_id: ActorId,
    pub instance_id: ActorInstanceId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_tx: Option<Sender<BoxMsgAny>>,
    pub controller_tx: Option<Sender<BoxMsgAny>>,
    pub ping_count: u64,
}

impl Actor for Client {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    fn get_instance_id(&self) -> &ActorInstanceId {
        &self.instance_id
    }

    fn get_protocol_set(&self) -> &ProtocolSet {
        &self.protocol_set
    }

    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny) {
        (self.current_state)(self, reply_tx, msg);
    }
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

// From: https://www.uuidgenerator.net/version4
const CLIENT_ACTOR_ID: ActorId = ActorId(uuid!("02960323-48ef-4e9e-b3b7-d8a3ad6b49ed"));
const CLIENT_PROTOCOL_SET_ID: ProtocolSetId =
    ProtocolSetId(uuid!("1a7b43ed-4676-42cd-9969-72283f258ef1"));

impl Client {
    pub fn new(name: &str) -> Self {
        // Create the client ProtocolSet, `client_ps`
        let errp = echo_requester_protocol();
        let erep = echo_requestee_protocol();
        let escp = echo_start_complete_protocol();
        let mut client_pm = HashMap::<ProtocolId, Protocol>::new();
        client_pm.insert(errp.id.clone(), errp.clone());
        client_pm.insert(erep.id.clone(), erep.clone());
        client_pm.insert(escp.id.clone(), escp.clone());

        let client_ps = ProtocolSet::new("client_ps", CLIENT_PROTOCOL_SET_ID, client_pm);
        let mut this = Self {
            name: name.to_owned(),
            actor_id: CLIENT_ACTOR_ID,
            instance_id: ActorInstanceId::new(),
            protocol_set: client_ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            partner_tx: None,
            controller_tx: None,
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

    fn send_echo_req_or_complete(&mut self, counter: u64) {
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
            if let Some(tx) = &self.partner_tx {
                tx.send(req_msg).unwrap();
            } else {
                println!(
                    "{}:send_echo_req_or_complete:- no partner={req_msg:?}",
                    self.name
                );
            }
        } else if let Some(tx) = &self.controller_tx {
            tx.send(Box::new(EchoComplete::new())).unwrap();
            println!(
                "{}:send_echo_req_or_complete:- send Complete to controller_tx",
                self.name
            );
        } else {
            println!("{}:send_echo_req_or_complete:- no controller_tx", self.name);
        }
    }

    pub fn state0(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReply>() {
            assert_eq!(msg.header.id, ECHO_REPLY_ID);
            println!("{}:State0: {msg:?}", self.name);
            self.send_echo_req_or_complete(msg.counter + 1);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            let reply_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
            if let Some(tx) = reply_tx {
                tx.send(reply_msg).unwrap();
            } else {
                println!(
                    "{}:State0: Error no reply_tx, can't send repl_msg={reply_msg:?}",
                    self.name
                );
            }
        } else if let Some(msg) = msg_any.downcast_ref::<EchoStart>() {
            assert_eq!(msg.header.id, ECHO_START_ID);
            if let Some(tx) = reply_tx {
                println!("{}:State0: msg={msg:?}", self.name);
                self.partner_tx = Some(msg.partner_tx.clone());
                self.controller_tx = Some(tx.clone());
                self.ping_count = msg.ping_count;
                self.send_echo_req_or_complete(1);
            } else {
                println!(
                    "{}:State0: Error no controller_tx, can't start msg={msg:?}",
                    self.name
                );
            }
        } else {
            let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
            println!(
                "{}:State0: Unknown msg_any={msg_any:?} {msg_id:?}",
                self.name
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use std::sync::mpsc::channel;

    // Test various ping_counts including 0
    #[test]
    fn test_ping_counts() {
        // Channel pair between ctrl and clnt
        let (ctrl_to_clnt_tx, ctrl_to_clnt_rx) = channel::<BoxMsgAny>();
        let (clnt_to_ctrl_tx, clnt_to_ctrl_rx) = channel::<BoxMsgAny>();

        // Channel pair between clnt and srvr
        let (clnt_to_srvr_tx, clnt_to_srvr_rx) = channel::<BoxMsgAny>();
        let (srvr_to_clnt_tx, srvr_to_clnt_rx) = channel::<BoxMsgAny>();

        let mut client = Client::new("client");
        println!("test_ping_counts: client={client:?}");

        for ping_count in [0, 1, 5] {
            // Controller sends start message to client
            println!("\ntest_ping_counts: ping_count={ping_count}");
            let start_msg = Box::new(EchoStart::new(clnt_to_srvr_tx.clone(), ping_count));
            ctrl_to_clnt_tx.send(start_msg).unwrap();

            // Client receives Start msg
            let start_msg_any = ctrl_to_clnt_rx.recv().unwrap();
            client.process_msg_any(Some(&clnt_to_ctrl_tx), start_msg_any);

            for _ in 0..ping_count {
                // Server receives request message
                let req_msg_any = clnt_to_srvr_rx.recv().unwrap();
                let req_msg = req_msg_any.downcast_ref::<EchoReq>().unwrap();
                println!("test_ping_counts: received req_msg={req_msg:?}");

                // Server creates and sends reply message
                let reply_msg = Box::new(EchoReply::new(
                    Utc::now().timestamp_nanos(),
                    req_msg.counter,
                ));
                srvr_to_clnt_tx.send(reply_msg).unwrap();

                // Client receives and processes reply message
                let reply_msg_any = srvr_to_clnt_rx.recv().unwrap();
                client.process_msg_any(Some(&srvr_to_clnt_tx), reply_msg_any);
            }

            // Controller receives Complete msg
            let complete_msg_any = clnt_to_ctrl_rx.recv().unwrap();
            let complete_msg = complete_msg_any.downcast_ref::<EchoComplete>().unwrap();
            println!("test_ping_counts: received complete msg={complete_msg:?}");
            assert_eq!(complete_msg.header.id, echo_complete::ECHO_COMPLETE_ID);
        }

        drop(ctrl_to_clnt_tx);
        drop(clnt_to_ctrl_tx);
        drop(clnt_to_srvr_tx);
        drop(srvr_to_clnt_tx);
    }
}
