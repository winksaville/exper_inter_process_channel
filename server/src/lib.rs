use actor::{Actor, ActorId, ActorInstanceId};
use crossbeam_channel::Sender;
use echo_requestee_protocol::{echo_requestee_protocol, EchoReply, EchoReq, ECHO_REQ_ID};
use protocol::{Protocol, ProtocolId};
use protocol_set::{ProtocolSet, ProtocolSetId};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
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

// State machine for channel to network
pub struct Server {
    pub name: String,
    pub actor_id: ActorId,
    pub instance_id: ActorInstanceId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,

    self_tx: Option<Sender<BoxMsgAny>>,
}

impl Actor for Server {
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
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny) {
        (self.current_state)(self, reply_tx, msg);
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
            "{} {{ name: {}, state_info_hash: {:?}; current_state: {state_name} }}",
            self.name, self.name, self.state_info_hash
        )
    }
}

// From: https://www.uuidgenerator.net/version4
const SERVER_ACTOR_ID: ActorId = ActorId(uuid!("d9a4c51e-c42e-4f2e-ae6c-96f62217d892"));
const SERVER_PROTOCOL_SET_ID: ProtocolSetId =
    ProtocolSetId(uuid!("4c797cb5-08ff-4970-9a6b-17c5d296f69f"));

impl Server {
    pub fn new(name: &str) -> Self {
        // Create the server ProtocolSet, `server_ps`.
        let erep = echo_requestee_protocol();
        let mut server_pm = HashMap::<ProtocolId, Protocol>::new();
        server_pm.insert(erep.id.clone(), erep.clone());
        let server_ps = ProtocolSet::new("server_ps", SERVER_PROTOCOL_SET_ID, server_pm);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: SERVER_ACTOR_ID,
            instance_id: ActorInstanceId::new(),
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

    pub fn state0(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.header.id, ECHO_REQ_ID);
            //println!("{}:State0: msg={msg:?}", self.name);
            let reply_msg = Box::new(EchoReply::new(msg.req_timestamp_ns, msg.counter));
            //println!("{}:State0: sending reply_msg={reply_msg:?}", self.name);
            let tx = reply_tx.unwrap();
            tx.send(reply_msg).unwrap();
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

    #[test]
    fn test_1() {
        let (tx, rx) = unbounded();

        let mut server = Server::new("server");
        println!("test_1: server={server:?}");

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
            server.process_msg_any(Some(&tx), echo_req_any);

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
