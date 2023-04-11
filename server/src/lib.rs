use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_channel::ActorChannel;
use an_id::{anid, paste, AnId};
use cmd_init_protocol::{cmd_init_protocol, CmdInit, CMD_INIT_ID};
use con_mgr_register_actor_protocol::{
    ConMgrRegisterActorReq, ConMgrRegisterActorRsp, ConMgrRegisterActorStatus,
    CON_MGR_REGISTER_ACTOR_RSP_ID,
};
use echo_requestee_protocol::{echo_requestee_protocol, EchoReq, EchoRsp, ECHO_REQ_ID};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use sender_map_by_instance_id::sender_map_insert;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use box_msg_any::BoxMsgAny;
use msg_header::MsgHeader;

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
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub chnl: ActorChannel,
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for Server {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for Server {}

impl Actor for Server {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_actor_id(&self) -> &AnId {
        &self.actor_id
    }

    fn get_instance_id(&self) -> &AnId {
        &self.instance_id
    }

    fn get_chnl(&self) -> &ActorChannel {
        &self.chnl
    }

    fn process_msg_any(&mut self, context: &dyn ActorContext, msg: BoxMsgAny) {
        (self.current_state)(self, context, msg);
    }

    fn done(&self) -> bool {
        false
    }
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
            "{} {{ id: {} instance_id: {} state_info_hash: {:?}; current_state: {state_name}; protocol_set: {:?}}}",
            self.name, self.actor_id, self.instance_id, self.state_info_hash, self.protocol_set
        )
    }
}

// From: https://www.uuidgenerator.net/version4
const SERVER_ACTOR_ID: AnId = anid!("d9a4c51e-c42e-4f2e-ae6c-96f62217d892");
const SERVER_PROTOCOL_SET_ID: AnId = anid!("4c797cb5-08ff-4970-9a6b-17c5d296f69f");

impl Server {
    pub fn new(name: &str) -> Self {
        // Create the server ProtocolSet, `server_ps`.
        println!("Server::new({})", name);
        let mut server_pm = HashMap::<AnId, Protocol>::new();
        let ci_protocol = cmd_init_protocol();
        server_pm.insert(ci_protocol.id, ci_protocol.clone());
        let erep = echo_requestee_protocol();
        server_pm.insert(erep.id, erep.clone());
        let server_ps = ProtocolSet::new("server_ps", SERVER_PROTOCOL_SET_ID, server_pm);

        let chnl_name = name.to_owned() + "_chnl";
        let chnl = ActorChannel::new(&chnl_name);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: SERVER_ACTOR_ID,
            instance_id: AnId::new(),
            protocol_set: server_ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            chnl,
        };

        // Add ourself to the sender_map
        sender_map_insert(&this.instance_id, &this.chnl.sender);

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

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.msg_id, ECHO_REQ_ID);
            println!("{}:State0: msg={msg:?}", self.name);
            let rsp_msg = Box::new(EchoRsp::new(
                &self.instance_id,
                msg.req_timestamp_ns,
                msg.counter,
            ));
            println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_rsp(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<CmdInit>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.header.msg_id, CMD_INIT_ID);

            // Register ourselves with ConMgr
            let msg = Box::new(ConMgrRegisterActorReq::new(
                &self.instance_id,
                &self.name,
                &self.actor_id,
                &self.instance_id,
                &self.protocol_set,
            ));
            print!(
                "{}:State0: sending ConMgrRegisterActorReq={msg:?}",
                self.name
            );
            context.send_con_mgr(msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<ConMgrRegisterActorRsp>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.header.msg_id, CON_MGR_REGISTER_ACTOR_RSP_ID);
            assert_eq!(msg.status, ConMgrRegisterActorStatus::Success);
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
    use actor_channel::ActorSender;
    use chrono::Utc;
    use con_mgr_register_actor_protocol::CON_MGR_REGISTER_ACTOR_REQ_ID;

    use super::*;

    struct Context {
        actor_executor_sender: ActorSender,
        con_mgr_tx: ActorSender,
        rsp_tx: ActorSender,
    }

    impl ActorContext for Context {
        fn actor_executor_tx(&self) -> &ActorSender {
            &self.actor_executor_sender
        }

        fn send_con_mgr(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.con_mgr_tx.send(msg)?)
        }

        fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn send_rsp(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.rsp_tx.send(msg)?)
        }

        fn clone_rsp_tx(&self) -> ActorSender {
            self.rsp_tx.clone()
        }
    }

    #[test]
    fn test_1() {
        println!("\ntest_1:+");

        let supervisor_chnl = ActorChannel::new("supervisor");

        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Context for server is supervisor
        let server_context = Context {
            actor_executor_sender: supervisor_chnl.sender.clone(),
            con_mgr_tx: supervisor_chnl.sender.clone(),
            rsp_tx: supervisor_chnl.sender.clone(),
        };

        let mut server = Server::new("server");
        println!("test_1: {server:?}");

        // Warm up reading time stamp
        let first_now_ns = Utc::now().timestamp_nanos();
        let second_now_ns = Utc::now().timestamp_nanos();
        let third_now_ns = Utc::now().timestamp_nanos();

        #[derive(Debug, Clone, Copy)]
        struct Times {
            now_ns: i64,
            req_timestamp_ns: i64,
            rsp_timestamp_ns: i64,
            last_ns: i64,
        }

        // We'll do 11 loop and throw out the first loop
        // as that is slow
        const LOOP_COUNT: usize = 10; //100;
        let zero_times = Times {
            now_ns: 0,
            req_timestamp_ns: 0,
            rsp_timestamp_ns: 0,
            last_ns: 0,
        };
        let mut times = [zero_times; LOOP_COUNT];

        for i in 0..LOOP_COUNT {
            // Mark start
            let now_ns = Utc::now().timestamp_nanos();

            // Create EchoReq and send it
            let echo_req: BoxMsgAny = Box::new(EchoReq::new(&supervisor_instance_id, 1));
            server.chnl.sender.send(echo_req).unwrap();

            // Receive EchoReq and process it in server
            let echo_req_any = server.chnl.receiver.recv().unwrap();
            server.process_msg_any(&server_context, echo_req_any);

            // Receive EchoRsp
            let rsp_msg_any = supervisor_chnl.receiver.recv().unwrap();
            let rsp_msg = rsp_msg_any.downcast_ref::<EchoRsp>().unwrap();

            // Mark done
            times[i].last_ns = Utc::now().timestamp_nanos();
            times[i].now_ns = now_ns;
            times[i].req_timestamp_ns = rsp_msg.req_timestamp_ns;
            times[i].rsp_timestamp_ns = rsp_msg.rsp_timestamp_ns;
        }

        // Display all times
        //println!("test_1: times {times:#?}");
        //println!();

        println!(
            "test_1:          second_now_ns - first_now_ns = {:6}ns",
            second_now_ns - first_now_ns
        );
        println!(
            "test_1:          third_now_ns - second_now_ns = {:6}ns",
            third_now_ns - second_now_ns
        );
        println!();

        let mut sum_t0 = 0;
        let mut sum_t1 = 0;
        let mut sum_t2 = 0;
        let mut sum_rtt = 0;
        let ignoring = LOOP_COUNT / 5;
        for i in 0..LOOP_COUNT {
            let t0 = times[i].req_timestamp_ns - times[i].now_ns;
            let t1 = times[i].rsp_timestamp_ns - times[i].req_timestamp_ns;
            let t2 = times[i].last_ns - times[i].rsp_timestamp_ns;
            let rtt = times[i].last_ns - times[i].now_ns;

            if i >= ignoring {
                // Not ignoring
                sum_t0 += t0;
                sum_t1 += t1;
                sum_t2 += t2;
                sum_rtt += rtt;
            }

            // Show the first and last few loops
            if (i == 0) || (i >= (LOOP_COUNT - 5)) {
                if i == 0 {
                    println!("First loop");
                } else {
                    println!("Loop {}", i + 1);
                }
                println!("  t0 = {:6}ns", t0);
                println!("  t1 = {:6}ns", t1);
                println!("  t2 = {:6}ns", t2);
                println!(" rtt = {:6}ns", rtt);
                println!();
            }
        }

        println!("Average times of the last {} loops", LOOP_COUNT - ignoring);
        let avg_count = (LOOP_COUNT - ignoring) as i64;
        println!("  t0 = {}ns", sum_t0 / avg_count);
        println!("  t1 = {}ns", sum_t1 / avg_count);
        println!("  t2 = {}ns", sum_t2 / avg_count);
        println!(" rtt = {}ns", sum_rtt / avg_count);

        println!("test_1:-");
    }

    #[test]
    fn test_cmd_init() {
        println!("\ntest_cmd_init:+");

        let supervisor_chnl = ActorChannel::new("supervisor");

        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Both con_mgr_tx and rsp_tx are "this" test
        let server_context = Context {
            actor_executor_sender: supervisor_chnl.sender.clone(),
            con_mgr_tx: supervisor_chnl.sender.clone(),
            rsp_tx: supervisor_chnl.sender.clone(),
        };

        let mut server = Server::new("server");

        // First message must be CmdInit and client send
        let msg = Box::new(CmdInit::new(&supervisor_instance_id));
        server.process_msg_any(&server_context, msg);

        // ConMgr is sent ConMgrRegisterActorReq and responds with
        // ConMgrRegisterActorRsp status: ConMgrRegisterActorStatus::Success
        let con_mgr_msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&con_mgr_msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let msg = Box::new(ConMgrRegisterActorRsp::new(
            &supervisor_instance_id,
            ConMgrRegisterActorStatus::Success,
        ));
        server.process_msg_any(&server_context, msg);

        println!("test_cmd_init:-");
    }
}
