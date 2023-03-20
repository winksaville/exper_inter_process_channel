use actor::{Actor, ActorContext, ProcessMsgFn};
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

    self_tx: Option<Sender<BoxMsgAny>>,
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
            "{} {{ name: {}, state_info_hash: {:?}; current_state: {state_name}; protocol_set: {:?} }}",
            self.name, self.name, self.state_info_hash, self.protocol_set
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
        let erep = echo_requestee_protocol();
        let mut server_pm = HashMap::<AnId, Protocol>::new();
        server_pm.insert(erep.id, erep.clone());
        let server_ps = ProtocolSet::new("server_ps", SERVER_PROTOCOL_SET_ID, server_pm);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: SERVER_ACTOR_ID,
            instance_id: AnId::new(),
            protocol_set: server_ps,
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
    use actor_bi_dir_channel::BiDirLocalChannel;
    use chrono::Utc;

    use super::*;

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
        let (server_bdlc, test_1_bdlc) = BiDirLocalChannel::new();

        let server_context = Context {
            their_bdlc_with_us: test_1_bdlc.clone(),
            rsp_tx: server_bdlc.tx.clone(),
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
            let echo_req: BoxMsgAny = Box::new(EchoReq::new(1));
            test_1_bdlc.tx.send(echo_req).unwrap();

            // Receive EchoReq and process it in server
            let echo_req_any = server_bdlc.rx.recv().unwrap();
            server.process_msg_any(&server_context, echo_req_any);

            // Receive EchoRsp
            let rsp_msg_any = test_1_bdlc.rx.recv().unwrap();
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
