use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_channel::{ActorChannel, ActorSender};
use an_id::{anid, paste, AnId};
use box_msg_any::BoxMsgAny;
use cmd_init_issuee_protocol::{cmd_init_issuee_protocol, CmdInit, CMD_INIT_ID};
use con_mgr_register_actor_protocol::{
    ConMgrRegisterActorReq, ConMgrRegisterActorRsp, ConMgrRegisterActorStatus,
    CON_MGR_REGISTER_ACTOR_RSP_ID,
};
use echo_requestee_protocol::echo_requestee_protocol;
use echo_requester_protocol::{
    echo_requester_protocol, EchoReq, EchoRsp, ECHO_REQ_ID, ECHO_RSP_ID,
};
use echo_start_complete_protocol::{
    echo_start_complete_protocol, EchoComplete, EchoStart, ECHO_START_ID,
};
use msg1::Msg1;
use msg2::Msg2;
use msg_header::MsgHeader;
use protocol::Protocol;
use protocol_set::ProtocolSet;
use sender_map_by_instance_id::{sender_map_get, sender_map_insert};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

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
/// After instantiating the Supervisor issues an EchoStart with a ping_count.
/// The Client will then ping the partner with an EchoReq and expects
/// the partner to respond with an EchoRsp. After pinging the expected
/// number of times the Client will repspond to the Supervisor with
/// EchoDone.
///
/// Errors and not handled gracefully, this is just demo.
pub struct Client {
    pub name: String,
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub chnl: ActorChannel,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_instance_id: Option<AnId>,
    pub partner_sndr: Option<ActorSender>,
    pub controller_instance_id: Option<AnId>,
    pub controller_sndr: Option<ActorSender>,
    pub ping_count: u64,
}

// TODO: For Send implementors must guarantee maybe moved between threads. ??
unsafe impl Send for Client {}

// TODO: This Sync guarantee is valid because multiple threads will never access an Actor. ??
unsafe impl Sync for Client {}

impl Actor for Client {
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
            "{} {{ id: {} instance_id: {} state_info_hash: {:?}; current_state: {state_name}; ping_count: {}; protocol_set: {:?}}}",
            self.name, self.actor_id, self.instance_id, self.state_info_hash, self.ping_count, self.protocol_set
        )
    }
}

// From: https://www.uuidgenerator.net/version4
const CLIENT_ACTOR_ID: AnId = anid!("02960323-48ef-4e9e-b3b7-d8a3ad6b49ed");
const CLIENT_PROTOCOL_SET_ID: AnId = anid!("1a7b43ed-4676-42cd-9969-72283f258ef1");

impl Client {
    pub fn new(name: &str) -> Self {
        // Create the client ProtocolSet, `client_ps`
        let mut client_pm = HashMap::<AnId, Protocol>::new();
        let ci_iep = cmd_init_issuee_protocol();
        let errp = echo_requester_protocol();
        let erep = echo_requestee_protocol();
        let escp = echo_start_complete_protocol();
        client_pm.insert(ci_iep.id, ci_iep.clone());
        client_pm.insert(errp.id, errp.clone());
        client_pm.insert(erep.id, erep.clone());
        client_pm.insert(escp.id, escp.clone());

        let client_ps = ProtocolSet::new("client_ps", CLIENT_PROTOCOL_SET_ID, client_pm);

        let client_instance_id = AnId::new();
        let chnl = ActorChannel::new(name, &client_instance_id);

        let mut this = Self {
            name: name.to_owned(),
            actor_id: CLIENT_ACTOR_ID,
            instance_id: client_instance_id,
            protocol_set: client_ps,
            current_state: Self::state0,
            chnl,
            state_info_hash: StateInfoMap::<Self>::new(),
            partner_instance_id: None,
            partner_sndr: None,
            controller_instance_id: None,
            controller_sndr: None,
            ping_count: 0,
        };

        // Add ourself to the sender_map
        sender_map_insert(&this.instance_id, &this.chnl.sender.clone());

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
            if let Some(sndr) = &self.partner_sndr {
                let req_msg = Box::new(EchoReq::new(
                    sndr.get_dst_instance_id(),
                    &self.instance_id,
                    counter,
                ));
                println!(
                    "{}:send_echo_req_or_complete:- to partner_tx msg={req_msg:?}",
                    self.name
                );
                sndr.send(req_msg).unwrap();
            } else {
                println!("{}:send_echo_req_or_complete:- no partner_tx", self.name);
            }
        } else if let Some(tx) = &self.controller_sndr {
            tx.send(Box::new(EchoComplete::new(
                tx.get_dst_instance_id(),
                &self.instance_id,
            )))
            .unwrap();
            println!(
                "{}:send_echo_req_or_complete:- send Complete to controller_tx",
                self.name
            );
        } else {
            println!("{}:send_echo_req_or_complete:- no controller_tx", self.name);
        }
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoRsp>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.msg_id(), &ECHO_RSP_ID);
            self.send_echo_req_or_complete(msg.counter + 1);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.msg_id(), &ECHO_REQ_ID);
            let rsp_msg = Box::new(EchoRsp::new(
                context.get_dst_instance_id(),
                &self.instance_id,
                msg.req_timestamp_ns,
                msg.counter,
            ));
            println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_dst(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoStart>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.msg_id(), &ECHO_START_ID);
            self.partner_instance_id = Some(msg.partner_instance_id);
            self.partner_sndr = sender_map_get(&self.partner_instance_id.unwrap());
            self.controller_instance_id = Some(*msg.src_id());
            self.controller_sndr = sender_map_get(&self.controller_instance_id.unwrap());
            self.ping_count = msg.ping_count;
            println!(
                "{}:State0: Successfully connected to partner start echoing",
                self.name
            );
            self.send_echo_req_or_complete(1);
        } else if let Some(msg) = msg_any.downcast_ref::<Msg2>() {
            // Got a Msg2 so self send a Msg1
            println!("{}:State0: {msg:?}", self.name);
            let msg1 = Box::new(Msg1::new(msg.src_id(), &self.instance_id, 123));
            context.send_dst(msg1).unwrap();
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
            context.send_con_mgr(msg).unwrap();
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
    use super::*;
    use chrono::Utc;
    use con_mgr_register_actor_protocol::{
        ConMgrRegisterActorRsp, ConMgrRegisterActorStatus, CON_MGR_REGISTER_ACTOR_REQ_ID,
    };
    use echo_start_complete_protocol::{EchoComplete, EchoStart, ECHO_COMPLETE_ID};
    use msg1::MSG1_ID;
    use msg_header::MsgHeader;

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
    fn test_cmd_init() {
        println!("\ntest_cmd_init:+");

        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        let supervisor_chnl = ActorChannel::new("supervisor", &supervisor_instance_id);
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Both con_mgr_tx and rsp_tx are supervisor
        let client_context = Context {
            ae_sndr: supervisor_chnl.sender.clone(),
            con_mgr_sndr: supervisor_chnl.sender.clone(),
            dst_sndr: supervisor_chnl.sender.clone(),
        };

        // Create a client
        let mut client = Client::new("client");

        // First message must be CmdInit and client send
        let msg = Box::new(CmdInit::new(
            client.get_instance_id(),
            &supervisor_instance_id,
        ));
        client.process_msg_any(&client_context, msg);

        // ConMgr is sent ConMgrRegisterActorReq and responds with
        // ConMgrRegisterActorRsp status: ConMgrRegisterActorStatus::Success
        let con_mgr_msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&con_mgr_msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let msg = Box::new(ConMgrRegisterActorRsp::new(
            client.get_instance_id(),
            &supervisor_instance_id,
            ConMgrRegisterActorStatus::Success,
        ));
        client.process_msg_any(&client_context, msg);

        // Send Msg2 expect Msg1 back
        let msg = Box::new(Msg2::new(client.get_instance_id(), &supervisor_instance_id));
        client.process_msg_any(&client_context, msg);
        let recv_msg = supervisor_chnl.receiver.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg),
            &MSG1_ID
        );

        println!("test_cmd_init:-");
    }

    // Test various ping_counts including 0
    #[test]
    fn test_client_ping_with_supervisor_as_server() {
        println!("\ntest_client_ping_with_supervisor_as_server:+");

        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        let supervisor_chnl = ActorChannel::new("supervisor", &supervisor_instance_id);
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Create a client with a supervisor as the actor_executor
        let mut client = Client::new("client");

        let supervisor_with_clnt_context = Context {
            ae_sndr: supervisor_chnl.sender.clone(),
            con_mgr_sndr: supervisor_chnl.sender.clone(),
            dst_sndr: supervisor_chnl.sender.clone(),
        };

        // First message must be CmdInit and client will send a ConMsgRegisterActorReq
        let msg = Box::new(CmdInit::new(
            client.get_instance_id(),
            &supervisor_instance_id,
        ));
        client.process_msg_any(&supervisor_with_clnt_context, msg);

        // ConMgr is sent ConMgrRegisterActorReq and responds with
        // ConMgrRegisterActorRsp status: ConMgrRegisterActorStatus::Success
        let con_mgr_msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&con_mgr_msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let msg = Box::new(ConMgrRegisterActorRsp::new(
            client.get_instance_id(),
            &supervisor_instance_id,
            ConMgrRegisterActorStatus::Success,
        ));
        client.process_msg_any(&supervisor_with_clnt_context, msg);

        // Server channel with the supervisor as the server
        let srvr_instance_id = AnId::new();
        let srvr_chnl = ActorChannel::new("server", &srvr_instance_id);
        sender_map_insert(&srvr_instance_id, &srvr_chnl.sender);

        for ping_count in [0, 1, 5] {
            let srvr_with_clnt_context = Context {
                ae_sndr: supervisor_chnl.sender.clone(),
                con_mgr_sndr: supervisor_chnl.sender.clone(),
                dst_sndr: client.chnl.sender.clone(),
            };

            // Supervisor sends EchoStart message to client
            println!("\ntest_client_ping_with_supervisor_as_server: ping_count={ping_count}");
            let start_msg = Box::new(EchoStart::new(
                client.get_instance_id(),
                &supervisor_instance_id,
                &srvr_instance_id,
                ping_count,
            ));
            client.chnl.sender.send(start_msg).unwrap();

            // Client receives EchoStart msg from supervisor
            println!("test_client_ping_with_supervisor_as_server: client receiving EchoStart");
            let start_msg_any = client.chnl.receiver.recv().unwrap();
            println!("test_client_ping_with_supervisor_as_server: client process EchoStart");
            client.process_msg_any(&supervisor_with_clnt_context, start_msg_any);

            for i in 0..ping_count {
                println!(
                    "test_client_ping_with_supervisor_as_server: server recv TOL {} of {ping_count}",
                    i + 1
                );

                // Server receives request message
                let req_msg_any = srvr_chnl.receiver.recv().unwrap();
                let req_msg = req_msg_any.downcast_ref::<EchoReq>().unwrap();
                println!(
                    "test_client_ping_with_supervisor_as_server: received req_msg={req_msg:?}"
                );

                // Server creates and sends rsp message
                let rsp_msg = Box::new(EchoRsp::new(
                    client.chnl.sender.get_dst_instance_id(),
                    &supervisor_instance_id,
                    Utc::now().timestamp_nanos(),
                    req_msg.counter,
                ));
                client.chnl.sender.send(rsp_msg).unwrap();

                // Client receives and processes rsp message from server
                let rsp_msg_any = client.chnl.receiver.recv().unwrap();
                client.process_msg_any(&srvr_with_clnt_context, rsp_msg_any);
            }

            // Supervisor receives Complete msg
            let complete_msg_any = supervisor_chnl.receiver.recv().unwrap();
            let complete_msg = complete_msg_any.downcast_ref::<EchoComplete>().unwrap();
            println!("test_client_ping_with_supervisor_as_server: received complete msg={complete_msg:?}");
            assert_eq!(complete_msg.msg_id(), &ECHO_COMPLETE_ID);
        }
        println!("test_client_ping_with_supervisor_as_server:-");
    }
}
