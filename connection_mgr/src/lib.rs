//! This file defines `struct BiDirLocalChannels` which is used
//! for bi-directional communication between actors. In particular
//! by defining a bi-directional channel an entity that invokes an
//! actors `process_msg_any` has available the `rsp_tx` and it reduces
//! the need to pass a `Channel` in messages. Most importantly when
//! an inter process communication channel is created directly passing
//! of a channel would not be possible, this will eliminate that need.
//!
//! Note: this is a separate file because it uses UnsafeCell.
use std::cell::UnsafeCell;

use crossbeam_channel::unbounded;

use actor::{Actor, ActorContext, ActorId, ActorInstanceId, ProcessMsgFn};
use actor_bi_dir_channel::BiDirLocalChannel;

use an_id::{anid, paste};
use crossbeam_channel::Sender;
use echo_requestee_protocol::{echo_requestee_protocol, EchoReply, EchoReq, ECHO_REQ_ID};
use protocol::{Protocol, ProtocolId};
use protocol_set::{ProtocolSet, ProtocolSetId};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use msg_header::{BoxMsgAny, MsgHeader};

#[derive(Debug, Clone)]
pub struct Connection {
    pub their_channel: BiDirLocalChannel,
    pub our_channel: BiDirLocalChannel,
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
            their_channel: BiDirLocalChannel {
                self_tx: right_tx.clone(),
                tx: left_tx.clone(),
                rx: left_rx,
            },
            our_channel: BiDirLocalChannel {
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
pub struct ConnectionMgr {
    pub name: String,
    pub actor_id: ActorId,
    pub instance_id: ActorInstanceId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,

    self_tx: Option<Sender<BoxMsgAny>>,
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for ConnectionMgr {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for ConnectionMgr {}

impl Actor for ConnectionMgr {
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

impl Debug for ConnectionMgr {
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

// From: https://www.uuidgenerator.net/version4
const CM_ACTOR_ID: ActorId = ActorId(anid!("3f82508e-7970-44e9-8fb9-b7936c9c4833"));
const CM_PROTOCOL_SET_ID: ProtocolSetId =
    ProtocolSetId(anid!("ea140384-faa7-4599-9f7d-dd4c2380a5fb"));

impl ConnectionMgr {
    pub fn new(name: &str) -> Self {
        // Create the ConnectionMgr ProtocolSet.
        println!("ConnectionMgr::new({})", name);
        let requestee_protocol = echo_requestee_protocol();
        let mut cm_pm = HashMap::<ProtocolId, Protocol>::new();
        cm_pm.insert(requestee_protocol.id.clone(), requestee_protocol.clone());
        let ps = ProtocolSet::new("connection_mgr_ps", CM_PROTOCOL_SET_ID, cm_pm);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: CM_ACTOR_ID,
            instance_id: ActorInstanceId::new(),
            protocol_set: ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
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

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            //println!("{}:State0: msg={msg:?}", self.name);
            let rsp_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            //println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
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
    use chrono::Utc;

    use super::*;

    use crossbeam_channel::unbounded;

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
    fn test_1() {
        let (tx, rx) = unbounded();

        let context = Context { rsp_tx: tx.clone() };
        let mut conn_mgr = ConnectionMgr::new("conn_mgr");
        println!("test_1: conn_mgr={conn_mgr:?}");

        // Warm up reading time stamp
        let first_now_ns = Utc::now().timestamp_nanos();
        let second_now_ns = Utc::now().timestamp_nanos();
        let third_now_ns = Utc::now().timestamp_nanos();

        #[derive(Debug, Clone, Copy)]
        struct Times {
            now_ns: i64,
            req_timestamp_ns: i64,
            reply_timestamp_ns: i64,
            last_ns: i64,
        }

        // We'll do 11 loop and throw out the first loop
        // as that is slow
        const LOOP_COUNT: usize = 100;
        let zero_times = Times {
            now_ns: 0,
            req_timestamp_ns: 0,
            reply_timestamp_ns: 0,
            last_ns: 0,
        };
        let mut times = [zero_times; LOOP_COUNT];

        for i in 0..LOOP_COUNT {
            // Mark start
            let now_ns = Utc::now().timestamp_nanos();

            // Create EchoReq and send it
            let echo_req: BoxMsgAny = Box::new(EchoReq::new(1));
            tx.send(echo_req).unwrap();

            // Receive EchoReq and process it in server
            let echo_req_any = rx.recv().unwrap();
            conn_mgr.process_msg_any(&context, echo_req_any);

            // Receive EchoReply
            let reply_msg_any = rx.recv().unwrap();
            let reply_msg = reply_msg_any.downcast_ref::<EchoReply>().unwrap();

            // Mark done
            times[i].last_ns = Utc::now().timestamp_nanos();
            times[i].now_ns = now_ns;
            times[i].req_timestamp_ns = reply_msg.req_timestamp_ns;
            times[i].reply_timestamp_ns = reply_msg.reply_timestamp_ns;
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
            let t1 = times[i].reply_timestamp_ns - times[i].req_timestamp_ns;
            let t2 = times[i].last_ns - times[i].reply_timestamp_ns;
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
        drop(tx);
    }
}
