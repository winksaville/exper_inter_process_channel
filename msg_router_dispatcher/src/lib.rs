use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_channel::ActorChannel;
use an_id::{anid, paste, AnId};
use cmd_init_protocol::{cmd_init_protocol, CmdInit, CMD_INIT_ID};
use con_mgr_register_actor_protocol::{
    ConMgrRegisterActorReq, ConMgrRegisterActorRsp, ConMgrRegisterActorStatus,
    CON_MGR_REGISTER_ACTOR_RSP_ID,
};
use crossbeam_channel::bounded;
use echo_requestee_protocol::{echo_requestee_protocol, EchoReq, EchoRsp, ECHO_REQ_ID};
use insert_key_msg_id_value_from_serde_json_buf_requestee_protocol::{
    insert_key_msg_id_value_from_serde_json_buf_requestee_protocol,
    InsertKeyMsgIdValueFromSerdeJsonBufReq, InsertKeyMsgIdValueFromSerdeJsonBufRsp,
    InsertKeyMsgIdValueFromSerdeJsonBufRspStatus,
};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use sender_map_by_instance_id::sender_map_insert;
use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Debug},
    io::Read,
    net::TcpListener,
    sync::{atomic::AtomicU64, Arc, RwLock},
    thread,
};
use utils::buf_u8_le_to_u16;

use box_msg_any::BoxMsgAny;
use msg_header::{get_msg_id_str_from_buf, FromSerdeJsonBuf, MsgHeader};

// State information
#[derive(Debug)]
pub struct StateInfo {
    pub name: String,
}

// HashMap that maps address of a ProcessMsgFn to StateInfo
type StateInfoMap<SM> = HashMap<*const ProcessMsgFn<SM>, StateInfo>;

// State machine for channel to network
pub struct MsgRouterDispatcher {
    pub name: String,
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub chnl: ActorChannel,
    pub addr: String, // IP Address of a msg-router-receiver
    pub insert_key_msg_id_value_from_serde_json_buf_map:
        Arc<RwLock<HashMap<String, FromSerdeJsonBuf>>>, // Map of MsgId of each message
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for MsgRouterDispatcher {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for MsgRouterDispatcher {}

impl Actor for MsgRouterDispatcher {
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

impl Debug for MsgRouterDispatcher {
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
const MSG_ROUTER_DISPATCHER_ACTOR_ID: AnId = anid!("c9079a70-d7d6-465c-96bb-3bd0a6b24294");
const MSG_ROUTER_DISPATCHER_PROTOCOL_SET_ID: AnId = anid!("d285f0a1-2b71-414e-935d-2559d4a02c3c");

impl MsgRouterDispatcher {
    pub fn new(name: &str, addr: &str) -> Self {
        // Create the msg_router ProtocolSet, `ps`.
        println!("MsgRouterDispatcher::new({})", name);
        let mut pm = HashMap::<AnId, Protocol>::new();
        let ci_protocol = cmd_init_protocol();
        pm.insert(ci_protocol.id, ci_protocol.clone());
        let erep = echo_requestee_protocol();
        pm.insert(erep.id, erep.clone());
        let md = insert_key_msg_id_value_from_serde_json_buf_requestee_protocol();
        pm.insert(md.id, md.clone());
        let msg_router_ps =
            ProtocolSet::new("msg_router_ps", MSG_ROUTER_DISPATCHER_PROTOCOL_SET_ID, pm);

        let msg_router_instance_id = AnId::new();
        let chnl_name = name.to_owned() + "_chnl";
        let chnl = ActorChannel::new(&chnl_name, &msg_router_instance_id);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: MSG_ROUTER_DISPATCHER_ACTOR_ID,
            instance_id: msg_router_instance_id,
            protocol_set: msg_router_ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            chnl,
            addr: addr.to_owned(),
            insert_key_msg_id_value_from_serde_json_buf_map: Arc::new(RwLock::new(HashMap::<
                String,
                FromSerdeJsonBuf,
            >::new(
            ))),
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

    pub fn add_msg_id_from_serde_json_buf(
        &mut self,
        msg_id: AnId,
        from_serde_json_buf: FromSerdeJsonBuf,
    ) -> bool {
        let insert_key_msg_id_value_from_serde_json_buf_map_clone =
            Arc::clone(&self.insert_key_msg_id_value_from_serde_json_buf_map);
        let mut wlocked_hashmap = insert_key_msg_id_value_from_serde_json_buf_map_clone
            .write()
            .unwrap(); // TODO: remove unwrap

        if let Entry::Vacant(e) = wlocked_hashmap.entry(msg_id.to_string()) {
            println!("add_msg_id_from_serde_json_buf: msg_id: {msg_id}");
            e.insert(from_serde_json_buf);

            true
        } else {
            false
        }
    }

    /// Reads messages from a TcpStream, deserializes them and sends them to an associated channel
    pub fn deserializer(&self) {
        println!("{}::deserializer:+", &self.name);
        let (status_tx, status_rx) = bounded::<String>(1);

        // Make copies of the data we need in the thread
        let self_name = self.name.clone();
        let deser_thread_addr = self.addr.clone();
        let deser_thread_insert_key_msg_id_value_from_serde_json_buf_map =
            Arc::clone(&self.insert_key_msg_id_value_from_serde_json_buf_map);
        thread::spawn(move || {
            println!("{}::deserializer_thread:+", &self_name);

            // Ignore errors for the moment
            let listener = TcpListener::bind(deser_thread_addr).unwrap();

            // Indicate we're ready
            status_tx.send("ready".to_owned()).unwrap_or_else(|_| {
                panic!(
                    "{}::deserializer_thread: Unable to indicate we're ready",
                    &self_name
                )
            });
            println!("{}::deserializer_thread: ready", &self_name);

            let stream_id = AtomicU64::new(0);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut tcp_stream) => {
                        // TODO: Make async, but for now spin up a separate thread for each connection
                        let inner_thread_id =
                            stream_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let deser_inner_thread_name = format!(
                            "{}::deserializer_inner_thread:{}",
                            self_name, inner_thread_id
                        );
                        let deser_inner_thread_insert_key_msg_id_value_from_serde_json_buf_map =
                            deser_thread_insert_key_msg_id_value_from_serde_json_buf_map.clone();
                        thread::spawn(move || {
                            //println!( "{}: stream:+", &deser_inner_thread_name);

                            loop {
                                // TODO: Probably need a signature and version indicator too.
                                let mut msg_len_buf = [0u8; 2];
                                if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
                                    println!(
                                        "{}: stream closed reading msg_len, stopping",
                                        &deser_inner_thread_name
                                    );
                                    break;
                                }

                                let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
                                //println!( "{}{inner_thread_id} msg_len={msg_len}", &deser_inner_thread_name);

                                // We need to initialize the Vec so read_exact knows how much to read.
                                // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
                                let mut msg_buf = vec![0; msg_len];
                                if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
                                    println!(
                                        "{}: stream close reading msg_buf, stopping",
                                        &deser_inner_thread_name
                                    );
                                    break;
                                }

                                let id_str = get_msg_id_str_from_buf(&msg_buf);
                                //println!("{}: mag.get({id_str}) lookup", &deser_inner_thread_name);
                                if let Ok(map) = deser_inner_thread_insert_key_msg_id_value_from_serde_json_buf_map.read() {
                                    println!("{}: deser_inner_thread_insert_key_msg_id_value_from_serde_json_buf_map, GOT lock. map.len={}", &deser_inner_thread_name, map.len());
                                    if let Some(fn_from_serde_json_buf) = map.get(id_str) {
                                        let box_msg_any =
                                            (*fn_from_serde_json_buf)(&msg_buf).unwrap();
                                        //println!(
                                        //    "{}: box_msg_any {:p} {} {box_msg_any:?}",
                                        //    &deser_inner_thread_name,
                                        //    box_msg_any,
                                        //    std::mem::size_of::<BoxMsgAny>()
                                        //);

                                        let sndr = MsgHeader::get_dst_sndr_from_boxed_msg_any(
                                            &box_msg_any,
                                        )
                                        .unwrap();
                                        match sndr.send(box_msg_any) {
                                            Ok(_) => (),
                                            Err(why) => {
                                                println!(
                                                    "{}: tx.send failed: {why}",
                                                    &deser_inner_thread_name
                                                );
                                            }
                                        }
                                    } else {
                                        println!(
                                            "{}: map.get({id_str}) NOT found",
                                            &deser_inner_thread_name
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}: deser_inner_thread_insert_key_msg_id_value_from_serde_json_buf_map, NO lock",
                                        &deser_inner_thread_name
                                    );
                                }
                            }
                            //println!( "{}:-", &deser_inner_thread_name);
                        });
                    }
                    Err(why) => {
                        println!(
                            "{}::deserializer_thread: Error accepting connection: {why}",
                            &self_name
                        );
                    }
                }
            }

            println!("{}::deserializer_thread:-", &self_name);
        });

        // Wait for outer thread to be running
        println!(
            "{}::deserializer: Wait for thread to be running",
            &self.name
        );
        status_rx
            .recv()
            .expect("{}::dserializer error, loop must have died");
        println!("{}::deserializer: thread running", &self.name);
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<InsertKeyMsgIdValueFromSerdeJsonBufReq>() {
            let msg_id = &msg.msg_id;
            let from_serde_json_buf: fn(&[u8]) -> Option<Box<dyn Any + Send>> =
                msg.from_serde_json_buf;
            let status = if self.add_msg_id_from_serde_json_buf(*msg_id, from_serde_json_buf) {
                InsertKeyMsgIdValueFromSerdeJsonBufRspStatus::Success
            } else {
                InsertKeyMsgIdValueFromSerdeJsonBufRspStatus::AlreadyInserted
            };
            let rsp_msg = Box::new(InsertKeyMsgIdValueFromSerdeJsonBufRsp::new(
                context.get_dst_instance_id(),
                &self.instance_id,
                msg_id,
                status,
            ));
            context.send_dst(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            assert_eq!(msg.msg_id(), &ECHO_REQ_ID);
            //println!("{}:State0: msg={msg:?}", self.name);
            let rsp_msg = Box::new(EchoRsp::new(
                context.get_dst_instance_id(),
                &self.instance_id,
                msg.req_timestamp_ns,
                msg.counter,
            ));
            //println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_dst(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<CmdInit>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.msg_id(), &CMD_INIT_ID);

            // Register ourselves with ConMgr
            let msg = Box::new(ConMgrRegisterActorReq::new(
                msg.src_id(),
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
            assert_eq!(msg.msg_id(), &CON_MGR_REGISTER_ACTOR_RSP_ID);
            assert_eq!(msg.status, ConMgrRegisterActorStatus::Success);

            println!("{}:State0: starting deserializer", self.name);
            self.deserializer();
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
    use std::{net::TcpStream, time::Duration}; //, io::Write};

    use actor_channel::ActorSender;
    use actor_executor::{
        add_actor_to_actor_executor_blocking, initialize_supervisor_con_mgr_actor_executor_blocking,
    };
    use chrono::Utc;
    use cmd_done::CmdDone;
    use echo_requestee_protocol::ECHO_RSP_ID;
    use sender_map_by_instance_id::sender_map_get;
    use utils::write_msg_buf_to_tcp_stream;

    use super::*;

    struct Context {
        ae_sndr: ActorSender,
        con_mgr_sndr: ActorSender,
        dst_sndr: ActorSender,
    }

    impl ActorContext for Context {
        fn actor_executor_sndr(&self) -> &ActorSender {
            &self.ae_sndr
        }

        fn send_con_mgr(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.con_mgr_sndr.send(msg)?)
        }

        fn get_con_mgr_instance_id(&self) -> &AnId {
            self.con_mgr_sndr.get_dst_instance_id()
        }

        fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn send_dst(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.dst_sndr.send(msg)?)
        }

        fn get_dst_instance_id(&self) -> &AnId {
            self.dst_sndr.get_dst_instance_id()
        }

        fn clone_dst_sndr(&self) -> ActorSender {
            self.dst_sndr.clone()
        }
    }

    #[test]
    fn test_1() {
        println!("\ntest_1:+");

        let mrd1_addr = "localhost:12345";

        // Initialize Supervisor starting a single ActorExecutor and the connection manager
        let (
            supervisor_instance_id,
            supervisor_chnl,
            ae_join_handle,
            ae_instance_id,
            con_mgr_instance_id,
        ) = initialize_supervisor_con_mgr_actor_executor_blocking();
        let _ae_sender = sender_map_get(&ae_instance_id).unwrap();

        // Add MsgRouterDispatcher to ActorExecutor
        let mrd1 = Box::new(MsgRouterDispatcher::new("mrd1", mrd1_addr));

        let (_mrd1_actor_id, mrd1_instance_id) = add_actor_to_actor_executor_blocking(
            mrd1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Context for msg_router is supervisor
        let _msg_router_context = Context {
            ae_sndr: supervisor_chnl.sender.clone(),
            con_mgr_sndr: supervisor_chnl.sender.clone(),
            dst_sndr: supervisor_chnl.sender.clone(),
        };
        println!("test_1: waiting 1ms to yield to mrd1 thread");
        thread::sleep(Duration::from_millis(1));

        // Add EchoReq to Deserializer msgs
        let msg = Box::new(InsertKeyMsgIdValueFromSerdeJsonBufReq::new(
            &supervisor_instance_id,
            &mrd1_instance_id,
            &ECHO_REQ_ID,
            EchoReq::from_serde_json_buf,
        ));
        sender_map_get(&mrd1_instance_id)
            .unwrap()
            .send(msg)
            .unwrap();

        // Connect to MsgRouterDispatcher
        let mut writer = TcpStream::connect(mrd1_addr).unwrap();

        let before_timestamp_ns = Utc::now().timestamp_nanos();

        // Create EchoReq message and serialize it
        let echo_msg = Box::new(EchoReq::new(
            &con_mgr_instance_id,
            &supervisor_instance_id,
            1,
        ));
        let buf = EchoReq::to_serde_json_buf(echo_msg).unwrap();

        // Send to MsgRouterDispatcher, aka. deserializer :)
        match write_msg_buf_to_tcp_stream(&mut writer, &buf) {
            Ok(_) => (),
            Err(why) => panic!("test_1: {why}"),
        }

        // Read EchoRsp that MsgRouterDispatcher sends to us the supervisor
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let after_timestamp_ns = Utc::now().timestamp_nanos();
        let msg = EchoRsp::from_box_msg_any(&msg_any).unwrap();
        assert_eq!(msg.msg_id(), &ECHO_RSP_ID);
        assert_eq!(msg.src_id(), &con_mgr_instance_id);
        assert_eq!(msg.dst_id(), &supervisor_instance_id);
        assert_eq!(msg.counter, 1);
        println!("test_1: msg={msg:?}");
        println!(
            "after - before = {}ns",
            after_timestamp_ns - before_timestamp_ns
        );
        println!(
            "rsp - before = {}ns",
            msg.req_timestamp_ns - before_timestamp_ns
        );
        println!(
            "rsp - req = {}ns",
            msg.rsp_timestamp_ns - msg.req_timestamp_ns
        );
        println!(
            "after - rsp = {}ns",
            after_timestamp_ns - msg.rsp_timestamp_ns
        );
        assert!(before_timestamp_ns < msg.req_timestamp_ns);
        assert!(msg.req_timestamp_ns < msg.rsp_timestamp_ns);
        assert!(msg.rsp_timestamp_ns < after_timestamp_ns);

        println!("test1: send CmdDone to ae");
        let msg = Box::new(CmdDone::new(&ae_instance_id, &supervisor_instance_id));
        sender_map_get(&ae_instance_id).unwrap().send(msg).unwrap();
        println!("test_1: sent CmdDone to ae");

        println!("test_1: ae_join_handle.join().unwrap()");
        ae_join_handle
            .join()
            .expect("Failed joining ae_join_handle");

        println!("test_1:-");
    }
}
