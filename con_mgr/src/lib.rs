//! Connection Manager
use std::error::Error;

use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_channel::{ActorChannel, ActorSender};
use cmd_init_protocol::{cmd_init_protocol, CmdInit, CMD_INIT_ID};
use con_mgr_query_protocol::{ConMgrQueryReq, ConMgrQueryRsp, CON_MGR_QUERY_REQ_ID};
use con_mgr_register_actor_protocol::{
    con_mgr_register_actor_protocol, ConMgrRegisterActorReq, ConMgrRegisterActorRsp,
    ConMgrRegisterActorStatus, CON_MGR_REGISTER_ACTOR_REQ_ID,
};

use an_id::{anid, paste, AnId};
use echo_requestee_protocol::{echo_requestee_protocol, EchoReq, EchoRsp, ECHO_REQ_ID};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use sender_map_by_instance_id::sender_map_get;
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
pub struct ConMgr {
    pub name: String,
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub chnl: ActorChannel,

    vec_of_actor_tx: Vec<ActorSender>,
    actors_map_by_instance_id: HashMap<AnId, usize>,
    actors_map_by_name: HashMap<String, Vec<usize>>,
    actors_map_by_id: HashMap<AnId, Vec<usize>>,
    actors_map_by_protocol_set_id: HashMap<AnId, Vec<usize>>,
    actors_map_by_protocol_id: HashMap<AnId, Vec<usize>>,
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for ConMgr {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for ConMgr {}

impl Actor for ConMgr {
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

impl Debug for ConMgr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fn_ptr = self.current_state as *const ProcessMsgFn<Self>;
        let fn_ptr_string = format!("{fn_ptr:p}");
        let state_name = if let Some(si) = self.state_info_hash.get(&fn_ptr) {
            // State does have a name
            si.name.as_str()
        } else {
            // State does NOT have a name, use address
            fn_ptr_string.as_str()
        };

        write!(
            f,
            "{} {{ id: {} instance_id: {} state_info_hash: {:?}; current_state: {state_name}",
            self.name, self.actor_id, self.instance_id, self.state_info_hash
        )?;

        write!(f, " vec_of_actor_tx: {:?},", self.vec_of_actor_tx,)?;
        write!(
            f,
            " actors_map_by_instance_id: {:?},",
            self.actors_map_by_instance_id
        )?;
        write!(f, " actors_map_by_name: {:?},", self.actors_map_by_name)?;
        write!(f, " actors_map_by_id: {:?},", self.actors_map_by_id)?;
        write!(
            f,
            " actors_map_by_protocol_set_id: {:?},",
            self.actors_map_by_protocol_set_id
        )?;
        write!(
            f,
            " actors_map_by_protocol_id: {:?} ",
            self.actors_map_by_protocol_id
        )?;
        write!(f, "}}",)
    }
}

// From: https://www.uuidgenerator.net/version4
const CON_MGR_ACTOR_ID: AnId = anid!("3f82508e-7970-44e9-8fb9-b7936c9c4833");
const CON_MGR_PROTOCOL_SET_ID: AnId = anid!("ea140384-faa7-4599-9f7d-dd4c2380a5fb");

impl ConMgr {
    pub fn new(name: &str) -> Self {
        // Create the ConMgr ProtocolSet.
        println!("ConMgr::new({})", name);
        let mut cm_pm = HashMap::<AnId, Protocol>::new();
        let ci_protocol = cmd_init_protocol();
        cm_pm.insert(ci_protocol.id, ci_protocol.clone());
        let requestee_protocol = echo_requestee_protocol();
        cm_pm.insert(requestee_protocol.id, requestee_protocol.clone());
        let con_mgr_reg_actor_protoocl = con_mgr_register_actor_protocol();
        cm_pm.insert(
            con_mgr_reg_actor_protoocl.id,
            con_mgr_reg_actor_protoocl.clone(),
        );
        let ps = ProtocolSet::new("con_mgr_ps", CON_MGR_PROTOCOL_SET_ID, cm_pm);

        let chnl = ActorChannel::new(name);

        println!("ConMgr::new({}):", name);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: CON_MGR_ACTOR_ID,
            instance_id: AnId::new(),
            protocol_set: ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            vec_of_actor_tx: Vec::new(),
            chnl,
            //actor_executor_instance_id: *actor_executor_instance_id,
            actors_map_by_instance_id: HashMap::new(),
            actors_map_by_name: HashMap::new(),
            actors_map_by_id: HashMap::new(),
            actors_map_by_protocol_id: HashMap::new(),
            actors_map_by_protocol_set_id: HashMap::new(),
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

    /// Add an Actor.
    pub fn add_actor(&mut self, msg: &ConMgrRegisterActorReq) -> Result<(), Box<dyn Error>> {
        println!("{}::add_actor:+ msg={msg:?}", self.name);

        let idx = self.vec_of_actor_tx.len();

        println!(
            "{}::add_actor: add_map_by_instance_id={} idx={idx}",
            self.name, msg.instance_id
        );
        if let Some(idx) = self.actors_map_by_instance_id.insert(msg.instance_id, idx) {
            println!("{}::add_actor: already added at idx={idx}", self.name);
            return Err(format!(
                "{}-{}::add_actor {} instance_id:{} : Actor already added at idx: {idx}",
                self.name, self.actor_id, msg.name, msg.instance_id
            )
            .into());
        }

        let actor_tx = sender_map_get(&msg.instance_id).unwrap();
        self.vec_of_actor_tx.push(actor_tx);

        self.add_map_by_name(idx, &msg.name);
        self.add_map_by_id(idx, &msg.id);
        self.add_map_by_protocol_set(idx, &msg.protocol_set);

        println!("{}::add_actor:- msg={msg:?}", self.name);
        Ok(())
    }

    fn add_map_by_name(&mut self, idx: usize, name: &str) {
        if let Some(v) = self.actors_map_by_name.get_mut(name) {
            // Add another actor with that name
            println!(
                "{}::add_map_by_name: another instance of name={name} push idx={idx}",
                self.name
            );
            v.push(idx);
        } else {
            // First time seeing this name, add to vector with one item
            println!(
                "{}::add_map_by_name: first instance of name={name} add vec with idx={idx}",
                self.name
            );
            self.actors_map_by_name.insert(name.to_owned(), vec![idx]);
        }
    }

    fn add_map_by_id(&mut self, idx: usize, id: &AnId) {
        if let Some(v) = self.actors_map_by_id.get_mut(id) {
            // Add another idx
            println!(
                "{}::add_map_by_id: another instance of id={id} push idx={idx}",
                self.name
            );
            v.push(idx);
        } else {
            // First time seeing this actor_id, add vector with one item
            println!(
                "{}::add_map_by_id: first instance of id={id} add vec with idx={idx}",
                self.name
            );
            self.actors_map_by_id.insert(*id, vec![idx]);
        }
    }

    fn add_map_by_protocol_set(&mut self, idx: usize, ps: &ProtocolSet) {
        if let Some(v) = self.actors_map_by_protocol_set_id.get_mut(&ps.id) {
            // Add another idx
            println!(
                "{}::add_map_by_protocol_id: another instance of protocol_set_id={} push idx={idx}",
                self.name, ps.id
            );
            v.push(idx);
        } else {
            // First time seeing this protocol_set, add vector with one item
            println!("{}::add_map_by_protocol_id: first instance of protocol_set_id={} add vec with idx={idx}", self.name, ps.id);
            self.actors_map_by_protocol_set_id.insert(ps.id, vec![idx]);

            self.add_map_by_protocol_id(idx, ps);
        }
    }

    fn add_map_by_protocol_id(&mut self, idx: usize, ps: &ProtocolSet) {
        let protocol_map = &ps.protocols_map;

        for k in protocol_map.keys() {
            if let Some(v) = self.actors_map_by_protocol_id.get_mut(k) {
                #[cfg(test)]
                println!("{}::add_map_by_protocol_id: another instance of protocol_id={k} push idx={idx}", self.name);
                v.push(idx);
            } else {
                // First time seeing this protocol_id, add vector with one item
                #[cfg(test)]
                println!("{}::add_map_by_protocol_id: first instance of protocol_id={k}, add vec with idx={idx}", self.name);
                self.actors_map_by_protocol_id.insert(*k, vec![idx]);
            }
        }
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<ConMgrRegisterActorReq>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.header.msg_id, CON_MGR_REGISTER_ACTOR_REQ_ID);
            let status = if self.add_actor(msg).is_ok() {
                ConMgrRegisterActorStatus::Success
            } else {
                ConMgrRegisterActorStatus::ActorAlreadyRegistered
            };

            println!("Sending ConMgrRegisterActorRsp");
            context
                .send_rsp(Box::new(ConMgrRegisterActorRsp::new(
                    &self.instance_id,
                    status,
                )))
                .unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<ConMgrQueryReq>() {
            println!(
                "{}:State0: msg={msg:?} TODO response is ALWAYS empty, fix!",
                self.name
            );
            assert_eq!(msg.header.msg_id, CON_MGR_QUERY_REQ_ID);
            context
                .send_rsp(Box::new(ConMgrQueryRsp::new(&self.instance_id, &[])))
                .unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            //println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.header.msg_id, ECHO_REQ_ID);
            let rsp_msg = Box::new(EchoRsp::new(
                &self.instance_id,
                msg.req_timestamp_ns,
                msg.counter,
            ));
            //println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_rsp(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<CmdInit>() {
            println!("{}:State0: {msg:?} nothing to do", self.name);
            assert_eq!(msg.header.msg_id, CMD_INIT_ID);
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
    use client::Client;
    use cmd_init_protocol::CmdInit;
    use con_mgr_register_actor_protocol::{
        CON_MGR_REGISTER_ACTOR_REQ_ID, CON_MGR_REGISTER_ACTOR_RSP_ID,
    };
    use echo_requestee_protocol::echo_requestee_protocol;
    use echo_requester_protocol::echo_requester_protocol;
    use echo_start_complete_protocol::echo_start_complete_protocol;
    use server::Server;
    struct Context {
        actor_executor_tx: ActorSender,
        con_mgr_tx: ActorSender,
        rsp_tx: ActorSender,
    }

    impl ActorContext for Context {
        fn actor_executor_tx(&self) -> &ActorSender {
            &self.actor_executor_tx
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

    fn issue_cmd_init(context: &mut Context, actor: &mut dyn Actor, sid: &AnId) {
        let msg = Box::new(CmdInit::new(sid));
        context.rsp_tx = sender_map_get(sid).unwrap();
        actor.process_msg_any(context, msg);
    }

    #[test]
    fn test_con_mgr_ping() {
        println!("\ntest_con_mgr_ping:+");
        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        let supervisor_chnl = ActorChannel::new("supervisor");
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        let mut context = Context {
            actor_executor_tx: supervisor_chnl.sender.clone(),
            con_mgr_tx: supervisor_chnl.sender.clone(),
            rsp_tx: supervisor_chnl.sender.clone(),
        };

        let mut con_mgr = ConMgr::new("con_mgr");
        println!("test_con_mgr_ping: con_mgr={con_mgr:?}");

        issue_cmd_init(&mut context, &mut con_mgr, &supervisor_instance_id);

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
        const LOOP_COUNT: usize = 100;
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
            con_mgr.chnl.sender.send(echo_req).unwrap();

            // Receive EchoReq and process it in server
            let echo_req_any = con_mgr.chnl.receiver.recv().unwrap();
            con_mgr.process_msg_any(&context, echo_req_any);

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
        //println!("test_con_mgr_ping: times {times:#?}");
        //println!();

        println!(
            "test_con_mgr_ping:          second_now_ns - first_now_ns = {:6}ns",
            second_now_ns - first_now_ns
        );
        println!(
            "test_con_mgr_ping:          third_now_ns - second_now_ns = {:6}ns",
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
        println!("test_con_mgr_ping:-");
    }

    #[test]
    fn test_reg_client_server() {
        println!("\ntest_reg_client_server:+");

        let supervisor_instance_id = AnId::new();
        let supervisor_chnl = ActorChannel::new("supervisor");
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Create connection manager
        let mut con_mgr = ConMgr::new("con_mgr");
        println!("test_reg_client_server: con_mgr={con_mgr:?}");

        // The context, but we'll modify rsp_tx for each actor
        let mut context = Context {
            actor_executor_tx: supervisor_chnl.sender.clone(),
            con_mgr_tx: con_mgr.chnl.sender.clone(),
            rsp_tx: supervisor_chnl.sender.clone(), // Will be updated after receiving msg
        };

        issue_cmd_init(&mut context, &mut con_mgr, &supervisor_instance_id);

        // Create client
        let mut client = Client::new("client");
        println!("test_reg_client_server: client={client:?}");
        issue_cmd_init(&mut context, &mut client, &supervisor_instance_id);

        // Create server
        let mut server = Server::new("server");
        println!("test_reg_client_server: client={client:?}");
        issue_cmd_init(&mut context, &mut server, &supervisor_instance_id);

        // Expect the client to have sent ConMgrRegisterActorReq to con_mgr
        let msg_any = con_mgr.chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(&msg_any);
        assert_eq!(src_id, &client.instance_id);
        println!(
            "test_reg_client_server: msg_any is CON_MGR_REGISTER_ACTOR_REQ_ID src_id is client"
        );

        // Update context and have ConMgr process message
        context.rsp_tx = sender_map_get(&src_id).unwrap();
        con_mgr.process_msg_any(&context, msg_any);

        // Expect the ConMgr to have sent ConMgrRegisterActorRsp to client with success
        let msg_any = client.chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_RSP_ID);
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(&msg_any);
        assert_eq!(src_id, &con_mgr.instance_id);
        println!(
            "test_reg_client_server: msg_any is CON_MGR_REGISTER_ACTOR_RSP_ID src_id is con_mgr"
        );

        // Update context and have Client process message
        context.rsp_tx = sender_map_get(&src_id).unwrap();
        client.process_msg_any(&context, msg_any);

        println!("test_reg_client_server: {con_mgr:?}");
        println!("test_reg_client_server:  {client:?}");

        // Validate con_mgr has processed the ConMgrRegsiterActor from client
        assert_eq!(con_mgr.vec_of_actor_tx.len(), 1);
        assert_eq!(con_mgr.actors_map_by_instance_id.len(), 1);
        assert_eq!(
            *con_mgr
                .actors_map_by_instance_id
                .get(&client.instance_id)
                .unwrap(),
            0
        );
        assert_eq!(con_mgr.actors_map_by_name.len(), 1);
        assert_eq!(
            con_mgr.actors_map_by_name.get(&client.name).unwrap(),
            &vec![0]
        );
        assert_eq!(con_mgr.actors_map_by_id.len(), 1);
        assert_eq!(
            con_mgr.actors_map_by_id.get(&client.actor_id).unwrap(),
            &vec![0]
        );
        assert_eq!(con_mgr.actors_map_by_protocol_set_id.len(), 1);
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_set_id
                .get(&client.protocol_set.id)
                .unwrap(),
            &vec![0]
        );
        assert_eq!(con_mgr.actors_map_by_protocol_id.len(), 4);
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&cmd_init_protocol().id)
                .unwrap(),
            &vec![0]
        );
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&echo_requester_protocol().id)
                .unwrap(),
            &vec![0]
        );
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&echo_requestee_protocol().id)
                .unwrap(),
            &vec![0]
        );
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&echo_start_complete_protocol().id)
                .unwrap(),
            &vec![0]
        );

        // Expect the server to have sent ConMgrRegisterActorReq to con_mgr
        let msg_any = con_mgr.chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(&msg_any);
        assert_eq!(src_id, &server.instance_id);
        println!(
            "test_reg_client_server: msg_any is CON_MGR_REGISTER_ACTOR_REQ_ID src_id is server"
        );

        // Update context and have ConMgr process message
        context.rsp_tx = sender_map_get(&src_id).unwrap();
        con_mgr.process_msg_any(&context, msg_any);

        // Expect the ConMgr to have sent ConMgrRegisterActorRsp to server with success
        let msg_any = server.chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_RSP_ID);
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(&msg_any);
        assert_eq!(src_id, &con_mgr.instance_id);
        println!(
            "test_reg_client_server: msg_any is CON_MGR_REGISTER_ACTOR_RSP_ID src_id is con_mgr"
        );

        // Update context and have Server process message
        context.rsp_tx = sender_map_get(&src_id).unwrap();
        server.process_msg_any(&context, msg_any);

        println!("test_reg_client_server: {con_mgr:?}");
        println!("test_reg_client_server:  {server:?}");

        // Validate con_mgr has processed the ConMgrRegsiterActor
        assert_eq!(con_mgr.vec_of_actor_tx.len(), 2);
        assert_eq!(con_mgr.actors_map_by_instance_id.len(), 2);
        assert_eq!(
            *con_mgr
                .actors_map_by_instance_id
                .get(&server.instance_id)
                .unwrap(),
            1
        );
        assert_eq!(con_mgr.actors_map_by_name.len(), 2);
        assert_eq!(
            con_mgr.actors_map_by_name.get(&server.name).unwrap(),
            &vec![1]
        );
        assert_eq!(con_mgr.actors_map_by_id.len(), 2);
        assert_eq!(
            con_mgr.actors_map_by_id.get(&server.actor_id).unwrap(),
            &vec![1]
        );
        assert_eq!(con_mgr.actors_map_by_protocol_set_id.len(), 2);
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_set_id
                .get(&server.protocol_set.id)
                .unwrap(),
            &vec![1]
        );
        assert_eq!(con_mgr.actors_map_by_protocol_id.len(), 4);
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&cmd_init_protocol().id)
                .unwrap(),
            &vec![0, 1]
        );
        assert_eq!(
            con_mgr
                .actors_map_by_protocol_id
                .get(&echo_requestee_protocol().id)
                .unwrap(),
            &vec![0, 1]
        );

        println!("test_reg_client_server:-");
    }
}
