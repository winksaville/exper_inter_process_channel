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
use insert_key_msg_id_value_to_serde_json_buf_requestee_protocol::{
    insert_key_msg_id_value_to_serde_json_buf_requestee_protocol,
    InsertKeyMsgIdValueToSerdeJsonBufReq, InsertKeyMsgIdValueToSerdeJsonBufRsp,
    InsertKeyMsgIdValueToSerdeJsonBufRspStatus,
};
use msg_router_forwarder_actor_sender_requestee_protocol::{
    MsgRouterForwarderActorSenderReq, MsgRouterForwarderActorSenderRsp,
};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use sender_map_by_instance_id::sender_map_insert;
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Debug},
    net::TcpStream,
    sync::{Arc, RwLock},
    thread,
};
use utils::write_msg_buf_to_tcp_stream;

use box_msg_any::BoxMsgAny;
use msg_header::{MsgHeader, ToSerdeJsonBuf};

// State information
#[derive(Debug)]
pub struct StateInfo {
    pub name: String,
}

// HashMap that maps address of a ProcessMsgFn to StateInfo
type StateInfoMap<SM> = HashMap<*const ProcessMsgFn<SM>, StateInfo>;

// State machine for channel to network
pub struct MsgRouterForwarder {
    pub name: String,
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub chnl: ActorChannel,
    pub forwarder_name: String,
    pub forwarder_instance_id: AnId,
    pub forwarder_chnl: ActorChannel,
    pub addr: String, // IP Address and port of a msg-router-dispatcher this connects to
    pub map_key_msg_id_value_to_serde_json_buf: Arc<RwLock<HashMap<AnId, ToSerdeJsonBuf>>>, // Map of MsgId to ToSerdeJsonBuf for each message
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for MsgRouterForwarder {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for MsgRouterForwarder {}

impl Actor for MsgRouterForwarder {
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

impl Debug for MsgRouterForwarder {
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
const MSG_ROUTER_RECEIVER_ACTOR_ID: AnId = anid!("31408435-3d0a-400d-83fd-a649c1321f7b");
const MSG_ROUTER_RECEIVER_PROTOCOL_SET_ID: AnId = anid!("cbfbb7cc-d5bd-41be-9a0b-30bafb518be2");

impl MsgRouterForwarder {
    pub fn new(name: &str, addr: &str) -> Self {
        // Create the msg_router ProtocolSet, `ps`.
        println!("MsgRouterforwarder::new({})", name);
        let mut pm = HashMap::<AnId, Protocol>::new();
        let ci_protocol = cmd_init_protocol();
        pm.insert(ci_protocol.id, ci_protocol.clone());
        let erep = echo_requestee_protocol();
        pm.insert(erep.id, erep.clone());
        let md = insert_key_msg_id_value_to_serde_json_buf_requestee_protocol();
        pm.insert(md.id, md.clone());
        let msg_router_ps = ProtocolSet::new(
            "msg_router_receiver_ps",
            MSG_ROUTER_RECEIVER_PROTOCOL_SET_ID,
            pm,
        );

        let msg_router_instance_id = AnId::new();
        let chnl_name = name.to_owned() + "_chnl";
        let chnl = ActorChannel::new(&chnl_name, &msg_router_instance_id);
        let forwarder_instance_id = AnId::new();
        let forwarder_name = chnl_name + "forwarder_chnl";
        let forwarder_chnl = ActorChannel::new(&forwarder_name, &forwarder_instance_id);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: MSG_ROUTER_RECEIVER_ACTOR_ID,
            instance_id: msg_router_instance_id,
            protocol_set: msg_router_ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            chnl,
            forwarder_name,
            forwarder_instance_id,
            forwarder_chnl,
            addr: addr.to_owned(),
            map_key_msg_id_value_to_serde_json_buf: Arc::new(RwLock::new(HashMap::<
                AnId,
                ToSerdeJsonBuf,
            >::new())),
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

    pub fn add_msg_id_to_serde_json_buf(
        &mut self,
        msg_id: AnId,
        to_serde_json_buf: ToSerdeJsonBuf,
    ) -> bool {
        let arc_clone_map_key_msg_id_value_to_serde_json_buf =
            Arc::clone(&self.map_key_msg_id_value_to_serde_json_buf);
        let mut wlocked_hashmap = arc_clone_map_key_msg_id_value_to_serde_json_buf
            .write()
            .unwrap(); // TODO: remove unwrap

        if let Entry::Vacant(e) = wlocked_hashmap.entry(msg_id) {
            println!("add_msg_id_to_serde_json_buf: msg_id: {msg_id}");
            e.insert(to_serde_json_buf);

            true
        } else {
            false
        }
    }

    /// Receive messages on a channel, serializes them and then writes them to TcpStream
    pub fn forwarder(&self) {
        println!("{}::forwarder:+", &self.name);
        let (status_tx, status_rx) = bounded(1);
        let self_name = self.name.clone();
        let self_addr = self.addr.clone();
        let self_forwarder_chnl_receiver = self.forwarder_chnl.receiver.clone();
        let arc_clone_map_key_msg_id_value_to_serde_json_buf =
            Arc::clone(&self.map_key_msg_id_value_to_serde_json_buf);
        thread::spawn(move || {
            println!("{}::forwarder_thread:+", &self_name);

            // Indicate ready to receive messages
            status_tx.send(()).unwrap_or_else(|_| {
                panic!(
                    "{}::forwarder_thread: erroring sending status ready",
                    &self_name
                )
            });

            // Ignore errors for the moment
            let mut writer = TcpStream::connect(self_addr).unwrap();

            println!("{}::forwarder_thread: Waiting  BoxMsgAny", &self_name);
            while let Ok(msg) = self_forwarder_chnl_receiver.recv() {
                println!("{}::forwarder_thread: Received msg", &self_name);

                let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg);
                if let Ok(map) = arc_clone_map_key_msg_id_value_to_serde_json_buf.read() {
                    println!("{}: arc_clone_map_key_msg_id_value_to_serde_json_buf, GOT lock. map.len={}", &self_name, map.len());
                    if let Some(fn_to_serde_json_buf) = map.get(msg_id) {
                        let buf = (*fn_to_serde_json_buf)(msg).unwrap();
                        println!("{}: serialized msg buf.len()={}", &self_name, buf.len());
                        //println!("{}:                      buf={buf:x?}", &self_name);

                        match write_msg_buf_to_tcp_stream(&mut writer, &buf) {
                            Ok(_) => {
                                println!("{}: successfully wrote msg to tcp_stream", &self_name)
                            }
                            Err(why) => panic!("{}::forwarder_thread: {why}", &self_name),
                        }
                    } else {
                        println!("{}: map.get({msg_id}) NOT found", &self_name);
                    }
                }
            }
            println!("{}::forwarder_thread:-", &self_name);
        });

        // Wait for thread to be running
        println!(
            "{}::forwarder: Waiting for thread to be running",
            &self.name
        );
        status_rx
            .recv()
            .expect("{}::forwarder error, loop must have died");
        println!("{}::forwarder:- thread running", &self.name);
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<InsertKeyMsgIdValueToSerdeJsonBufReq>() {
            let msg_id = &msg.msg_id;
            let to_serde_json_buf: fn(BoxMsgAny) -> Option<Vec<u8>> = msg.to_serde_json_buf;
            let status = if self.add_msg_id_to_serde_json_buf(*msg_id, to_serde_json_buf) {
                InsertKeyMsgIdValueToSerdeJsonBufRspStatus::Success
            } else {
                InsertKeyMsgIdValueToSerdeJsonBufRspStatus::AlreadyInserted
            };
            let rsp_msg = Box::new(InsertKeyMsgIdValueToSerdeJsonBufRsp::new(
                context.get_dst_instance_id(),
                &self.instance_id,
                msg_id,
                status,
            ));
            context.send_dst(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<MsgRouterForwarderActorSenderReq>() {
            let _instance_id = &msg.instance_id;
            let rsp_msg = Box::new(MsgRouterForwarderActorSenderRsp::new(
                context.get_dst_instance_id(),
                &self.instance_id,
                &self.forwarder_chnl.sender.clone(),
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

            println!("{}:State0: starting serializer", self.name);
            self.forwarder();
        } else if let Some(msg) = msg_any.downcast_ref::<ConMgrRegisterActorRsp>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.msg_id(), &CON_MGR_REGISTER_ACTOR_RSP_ID);
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
    use std::{io::Read, net::TcpListener};

    //use actor_channel::ActorSender;
    use actor_executor::{
        add_actor_to_actor_executor_blocking, initialize_supervisor_con_mgr_actor_executor_blocking,
    };
    use cmd_done::CmdDone;
    //use echo_requestee_protocol::ECHO_RSP_ID;
    use sender_map_by_instance_id::sender_map_get;
    use utils::buf_u8_le_to_u16;

    use super::*;

    #[test]
    fn test_1() {
        println!("\ntest_1:+");

        // Create a mock MsgRouterDispatcher
        let mock_mrd_addr = "localhost:12345";
        let mock_mrd_listener = TcpListener::bind(mock_mrd_addr).unwrap();

        // Initialize Supervisor starting a single ActorExecutor and the connection manager
        let (
            supervisor_instance_id,
            supervisor_chnl,
            ae_join_handle,
            ae_instance_id,
            con_mgr_instance_id,
        ) = initialize_supervisor_con_mgr_actor_executor_blocking();
        let _ae_sender = sender_map_get(&ae_instance_id).unwrap();

        // Add MsgRouterforwarder to ActorExecutor
        let mrr1 = Box::new(MsgRouterForwarder::new("mrr1", mock_mrd_addr));

        let (_mrd1_actor_id, mrr1_instance_id) = add_actor_to_actor_executor_blocking(
            mrr1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Add EchoReq to Serializer msgs
        let msg = Box::new(InsertKeyMsgIdValueToSerdeJsonBufReq::new(
            &mrr1_instance_id,
            &supervisor_instance_id,
            &ECHO_REQ_ID,
            EchoReq::to_serde_json_buf,
        ));
        sender_map_get(&mrr1_instance_id)
            .unwrap()
            .send(msg)
            .unwrap();

        println!("test_1: waiting for InsertKeyMsgIdValueToSerdeJsonBufRsp");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = InsertKeyMsgIdValueToSerdeJsonBufRsp::from_box_msg_any(&msg_any).unwrap();
        println!("test_1: msg={:?}", msg);

        // Get Use MsgRouterforwarder to get the MsgRouterforwarder ActorSender
        let msg = Box::new(MsgRouterForwarderActorSenderReq::new(
            &mrr1_instance_id,
            &supervisor_instance_id,
            &AnId::nil(), // Currently not used
        ));
        sender_map_get(&mrr1_instance_id)
            .unwrap()
            .send(msg)
            .unwrap();
        println!("test_1: waiting for MsgRouterforwarder");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = MsgRouterForwarderActorSenderRsp::from_box_msg_any(&msg_any).unwrap();
        println!("test_1: msg={:?}", msg);
        let forwarder = &msg.sender;

        // Use the forwarder to send a EchoReq which will serialize it and
        // we'll receive using mock_mrd_listener.
        println!("test_1: forward EchoReq");
        let echo_msg = Box::new(EchoReq::new(
            &con_mgr_instance_id,
            &supervisor_instance_id,
            1,
        ));
        forwarder.send(echo_msg.clone()).unwrap();
        println!("test_1: forward EchoReq={echo_msg:?}");

        // Now receive the msg from and verifiy it's echo_msg
        println!("test_1: wait for mrr1 to forward EchoReq");
        let (mut stream, _) = mock_mrd_listener.accept().unwrap();
        println!("test_1: got the stream whith contains the EchoReq");
        let msg_len_buf = &mut [0u8; 2];
        stream.read_exact(msg_len_buf).unwrap();
        let len = buf_u8_le_to_u16(msg_len_buf) as usize;
        println!("test_1: got the msg_len_buf={msg_len_buf:?} len={len}");
        let mut msg_buf = vec![0u8; len];
        stream.read_exact(msg_buf.as_mut_slice()).unwrap();
        println!("test_1: got EchoReq msg_buf.len()={}", msg_buf.len());
        //println!("test_1:                 msg_buf={msg_buf:x?}");
        let box_msg_any = EchoReq::from_serde_json_buf(&msg_buf).unwrap();
        let msg = box_msg_any.downcast::<EchoReq>().unwrap();
        println!("test_1: got EchoReq msg={msg:?}");
        assert_eq!((*msg).header, echo_msg.header);
        assert_eq!((*msg).req_timestamp_ns, echo_msg.req_timestamp_ns);
        assert_eq!((*msg).counter, echo_msg.counter);

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
