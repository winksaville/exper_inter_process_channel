//! This file defines `struct BiDirLocalChannels` which is used
//! for bi-directional communication between actors. In particular
//! by defining a bi-directional channel an entity that invokes an
//! actors `process_msg_any` has available the `rsp_tx` and it reduces
//! the need to pass a `Channel` in messages. Most importantly when
//! an inter process communication channel is created directly passing
//! of a channel would not be possible, this will eliminate that need.
//!
//! Note: this is a separate file because it uses UnsafeCell.
use std::{cell::UnsafeCell, error::Error};

use con_mgr_register_actor::{
    con_mgr_register_actor_protocol, ConMgrRegisterActorReq, ConMgrRegisterActorRsp,
    ConMgrRegisterActorStatus,
};
use crossbeam_channel::unbounded;

use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_bi_dir_channel::BiDirLocalChannel;

use an_id::{anid, paste, AnId};
use crossbeam_channel::Sender;
use echo_requestee_protocol::{echo_requestee_protocol, EchoReq, EchoRsp, ECHO_REQ_ID};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use msg_header::{BoxMsgAny, MsgHeader};

#[derive(Debug, Clone)]
pub struct Connection {
    pub their_bdlc_with_us: BiDirLocalChannel,
    pub our_bdlc_with_them: BiDirLocalChannel,
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

impl Connection {
    pub fn new() -> Self {
        // left_tx -----> right_rx
        let (left_tx, right_rx) = unbounded();

        // left_rx <---- right_tx
        let (right_tx, left_rx) = unbounded();

        Self {
            their_bdlc_with_us: BiDirLocalChannel {
                self_tx: right_tx.clone(),
                tx: left_tx.clone(),
                rx: left_rx,
            },
            our_bdlc_with_them: BiDirLocalChannel {
                self_tx: left_tx,
                tx: right_tx,
                rx: right_rx,
            },
        }
    }
}

#[derive(Debug)]
pub struct VecConnection(UnsafeCell<Vec<Connection>>);

impl Default for VecConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl VecConnection {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    // Panic's if idx is out of bounds
    pub fn get(&self, idx: usize) -> &Connection {
        unsafe {
            let v = &*self.0.get();
            &v[idx]
        }
    }

    pub fn push(&self, bdlcs: Connection) {
        unsafe {
            let ptr = &mut *self.0.get();
            ptr.push(bdlcs);
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let v = &*self.0.get();
            v.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

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

    self_tx: Option<Sender<BoxMsgAny>>,
    vec_of_actor_bdlc: Vec<BiDirLocalChannel>,
    actors_map_by_instance_id: HashMap<AnId, usize>,
    actors_map_by_name: HashMap<String, Vec<usize>>,
    actors_map_by_id: HashMap<AnId, Vec<usize>>,
    actors_map_by_protocol_id: HashMap<AnId, Vec<usize>>,
    actors_map_by_protocol_set_id: HashMap<AnId, Vec<usize>>,
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

impl Debug for ConMgr {
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
            "{} {{ name: {}, state_info_hash: {:?}; current_state: {state_name}, actors:  }}",
            self.name,
            self.name,
            self.state_info_hash, //self.actors
        )
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
        let requestee_protocol = echo_requestee_protocol();
        cm_pm.insert(requestee_protocol.id, requestee_protocol.clone());
        let con_mgr_reg_actor_protoocl = con_mgr_register_actor_protocol();
        cm_pm.insert(
            con_mgr_reg_actor_protoocl.id,
            con_mgr_reg_actor_protoocl.clone(),
        );
        let ps = ProtocolSet::new("con_mgr_ps", CON_MGR_PROTOCOL_SET_ID, cm_pm);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: CON_MGR_ACTOR_ID,
            instance_id: AnId::new(),
            protocol_set: ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            self_tx: None,
            vec_of_actor_bdlc: Vec::new(),
            actors_map_by_instance_id: HashMap::new(),
            actors_map_by_name: HashMap::new(),
            actors_map_by_id: HashMap::new(),
            actors_map_by_protocol_id: HashMap::new(),
            actors_map_by_protocol_set_id: HashMap::new(),
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

    /// Add an Actor.
    pub fn add_actor(&mut self, msg: &ConMgrRegisterActorReq) -> Result<(), Box<dyn Error>> {
        #[cfg(debug)]
        println!("Clone only in debug configuration");
        #[cfg(debug)]
        let actor_clone_for_panic = actor.clone();

        let idx = self.vec_of_actor_bdlc.len();

        if let Some(idx) = self.actors_map_by_instance_id.insert(msg.instance_id, idx) {
            return Err(format!(
                "{}-{}::add_actor {} instance_id:{} : Actor already added at idx: {}",
                self.name, self.actor_id, msg.name, msg.instance_id, idx
            )
            .into());
        }

        self.vec_of_actor_bdlc.push(msg.bdlc.clone());

        self.add_map_by_name(idx, &msg.name);
        self.add_map_by_id(idx, &msg.id);
        self.add_map_by_protocol_set(idx, &msg.protocol_set);

        Ok(())
    }

    fn add_map_by_name(&mut self, idx: usize, name: &str) {
        if let Some(v) = self.actors_map_by_name.get_mut(name) {
            // Add another actor with that name
            v.push(idx);
        } else {
            // First time seeing this name, add to vector with one item
            self.actors_map_by_name.insert(name.to_owned(), vec![idx]);
        }
    }

    fn add_map_by_id(&mut self, idx: usize, id: &AnId) {
        if let Some(v) = self.actors_map_by_id.get_mut(id) {
            // Add another idx
            v.push(idx);
        } else {
            // First time seeing this actor_id, add vector with one item
            self.actors_map_by_id.insert(*id, vec![idx]);
        }
    }

    fn add_map_by_protocol_set(&mut self, idx: usize, ps: &ProtocolSet) {
        if let Some(v) = self.actors_map_by_protocol_set_id.get_mut(&ps.id) {
            // Add another idx
            v.push(idx);
        } else {
            // First time seeing this protocol_set, add vector with one item
            self.actors_map_by_protocol_set_id.insert(ps.id, vec![idx]);

            self.add_map_by_protocol_id(idx, ps);
        }
    }

    fn add_map_by_protocol_id(&mut self, idx: usize, ps: &ProtocolSet) {
        let protocol_map = &ps.protocols_map;

        for k in protocol_map.keys() {
            if let Some(v) = self.actors_map_by_protocol_id.get_mut(k) {
                v.push(idx);
            } else {
                // First time seeing this protocol_id, add vector with one item
                self.actors_map_by_protocol_id.insert(*k, vec![idx]);
            }
        }
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<ConMgrRegisterActorReq>() {
            println!("{}:State0: msg={msg:?}", self.name);
            let status = if self.add_actor(msg).is_ok() {
                ConMgrRegisterActorStatus::Success
            } else {
                ConMgrRegisterActorStatus::ActorAlreadyRegistered
            };
            context
                .send_rsp(Box::new(ConMgrRegisterActorRsp::new(status)))
                .unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            //println!("{}:State0: msg={msg:?}", self.name);
            let rsp_msg = Box::new(EchoRsp::new(msg.req_timestamp_ns, msg.counter));
            //println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_rsp(rsp_msg).unwrap();
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
    use actor_bi_dir_channel::ActorBiDirChannel;
    use chrono::Utc;

    struct Context {
        their_bdlc_with_us: BiDirLocalChannel,
        rsp_tx: Sender<BoxMsgAny>,
    }

    impl ActorContext for Context {
        fn their_bdlc_with_us(&self) -> BiDirLocalChannel {
            self.their_bdlc_with_us.clone()
        }

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
    fn test_1() {
        let (their_bdlc_with_us, our_bdlc_with_them) = BiDirLocalChannel::new();

        //let (tx, rx) = unbounded();

        let context = Context {
            their_bdlc_with_us: their_bdlc_with_us.clone(),
            rsp_tx: our_bdlc_with_them.tx.clone(),
        };
        let mut conn_mgr = ConMgr::new("conn_mgr");
        println!("test_1: conn_mgr={conn_mgr:?}");

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
            let echo_req: BoxMsgAny = Box::new(EchoReq::new(1));
            their_bdlc_with_us.send(echo_req).unwrap();

            // Receive EchoReq and process it in server
            let echo_req_any = our_bdlc_with_them.recv().unwrap();
            conn_mgr.process_msg_any(&context, echo_req_any);

            // Receive EchoRsp
            let rsp_msg_any = their_bdlc_with_us.recv().unwrap();
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
    }
}
