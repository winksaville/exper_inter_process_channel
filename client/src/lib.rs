use actor::{Actor, ActorContext, ActorId, ActorInstanceId, ProcessMsgFn};
use an_id::AnId;
use crossbeam_channel::Sender;
use echo_requestee_protocol::echo_requestee_protocol;
use echo_requester_protocol::{
    echo_requester_protocol, EchoReply, EchoReq, ECHO_REPLY_ID, ECHO_REQ_ID,
};
use echo_start_complete_protocol::{
    echo_start_complete_protocol, EchoComplete, EchoStart, ECHO_START_ID,
};
use msg1::Msg1;
use msg2::Msg2;
use protocol::{Protocol, ProtocolId};
use protocol_set::{ProtocolSet, ProtocolSetId};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};
use uuid::uuid;

use msg_header::{BoxMsgAny, MsgHeader};

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
    self_tx: Option<Sender<BoxMsgAny>>,
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for Client {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for Client {}

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

    fn set_self_sender(&mut self, sender: Sender<BoxMsgAny>) {
        self.self_tx = Some(sender);
    }

    fn process_msg_any(&mut self, context: &dyn ActorContext, msg: BoxMsgAny) {
        (self.current_state)(self, context, msg);
    }

    fn done(&self) -> bool {
        false
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
const CLIENT_ACTOR_ID: ActorId = ActorId(AnId(uuid!("02960323-48ef-4e9e-b3b7-d8a3ad6b49ed")));
const CLIENT_PROTOCOL_SET_ID: ProtocolSetId =
    ProtocolSetId(AnId(uuid!("1a7b43ed-4676-42cd-9969-72283f258ef1")));

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
            self_tx: None,
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

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReply>() {
            assert_eq!(msg.header.id, ECHO_REPLY_ID);
            println!("{}:State0: {msg:?}", self.name);
            self.send_echo_req_or_complete(msg.counter + 1);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            let rsp_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_rsp(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoStart>() {
            assert_eq!(msg.header.id, ECHO_START_ID);
            if let Some(tx) = context.clone_rsp_tx() {
                println!("{}:State0: msg={msg:?}", self.name);
                self.partner_tx = Some(msg.partner_tx.clone());
                self.controller_tx = Some(tx);
                self.ping_count = msg.ping_count;
                self.send_echo_req_or_complete(1);
            } else {
                println!(
                    "{}:State0: Error no controller_tx, can't start msg={msg:?}",
                    self.name
                );
            }
        } else if let Some(_msg) = msg_any.downcast_ref::<Msg2>() {
            // Got a Msg2 so self send a Msg1 so our test passes :)
            let msg1 = Box::<Msg1>::default();
            self.self_tx.as_ref().unwrap().send(msg1).unwrap();
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
    use actor_bi_dir_channel::{ActorBiDirChannel, BiDirLocalChannel};
    use chrono::Utc;
    use crossbeam_channel::unbounded;
    use echo_start_complete_protocol::{EchoComplete, EchoStart, ECHO_COMPLETE_ID};
    use msg1::MSG1_ID;
    use msg_header::MsgHeader;

    struct Context {
        rsp_tx: Sender<BoxMsgAny>,
    }

    impl ActorContext for Context {
        fn send_conn_mgr(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn send_rsp(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.rsp_tx.send(msg)?)
        }

        fn clone_rsp_tx(&self) -> Option<Sender<BoxMsgAny>> {
            Some(self.rsp_tx.clone())
        }
    }

    #[test]
    fn test_self_tx() {
        let (tx, rx) = unbounded::<BoxMsgAny>();

        let context = Context { rsp_tx: tx.clone() };

        let mut client = Client::new("client");
        client.set_self_sender(tx);

        let msg2 = Box::new(Msg2::new());
        client.process_msg_any(&context, msg2);
        let recv_msg = rx.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg),
            &MSG1_ID
        );
    }

    // Test various ping_counts including 0
    #[test]
    fn test_bi_dir_local_channel() {
        println!("test_bi_dir_local_channel:");

        // BiDirLocalChannel between ctrl and clnt
        let (ctrl_side_with_clnt, clnt_side_with_ctrl) = BiDirLocalChannel::new();

        // BiDirLocalChannel between clnt and srvr
        let (clnt_side_with_srvr, srvr_side_with_clnt) = BiDirLocalChannel::new();

        let mut client = Client::new("client");

        for ping_count in [0, 1, 5] {
            let clnt_side_with_ctrl_context = Context {
                rsp_tx: clnt_side_with_ctrl.clone_tx(),
            };

            let srvr_side_with_clnt_context = Context {
                rsp_tx: srvr_side_with_clnt.clone_tx(),
            };

            // Controller sends start message to client
            println!("\ntest_bi_dir_local_channel: ping_count={ping_count}");
            let tx = clnt_side_with_srvr.clone_tx();
            let start_msg = Box::new(EchoStart::new(tx, ping_count));
            ctrl_side_with_clnt.send(start_msg).unwrap();

            // Client receives Start msg from control
            let start_msg_any = clnt_side_with_ctrl.recv().unwrap();
            client.process_msg_any(&clnt_side_with_ctrl_context, start_msg_any);

            for i in 0..ping_count {
                println!(
                    "test_bi_dir_local_channel: server recv TOL {} of {ping_count}",
                    i + 1
                );

                // Server receives request message
                let req_msg_any = srvr_side_with_clnt.recv().unwrap();
                let req_msg = req_msg_any.downcast_ref::<EchoReq>().unwrap();
                println!("test_bi_dir_local_channel: received req_msg={req_msg:?}");

                // Server creates and sends reply message
                let reply_msg = Box::new(EchoReply::new(
                    Utc::now().timestamp_nanos(),
                    req_msg.counter,
                ));
                srvr_side_with_clnt.send(reply_msg).unwrap();

                // Client receives and processes reply message from server
                let reply_msg_any = clnt_side_with_srvr.recv().unwrap();
                client.process_msg_any(&srvr_side_with_clnt_context, reply_msg_any);
            }

            // Controller receives Complete msg
            let complete_msg_any = ctrl_side_with_clnt.recv().unwrap();
            let complete_msg = complete_msg_any.downcast_ref::<EchoComplete>().unwrap();
            println!("test_bi_dir_local_channel: received complete msg={complete_msg:?}");
            assert_eq!(complete_msg.header.id, ECHO_COMPLETE_ID);
        }
    }
}
