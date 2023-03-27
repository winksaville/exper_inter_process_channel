use actor::{Actor, ActorContext, ProcessMsgFn};
use actor_bi_dir_channel::Connection;
use an_id::{anid, paste, AnId};
use cmd_init_protocol::{cmd_init_protocol, CmdInit, CMD_INIT_ID};
use con_mgr_connect_protocol::{ConMgrConnectReq, ConMgrConnectRsp, CON_MGR_CONNECT_RSP_ID, ConMgrConnectStatus};
use con_mgr_register_actor_protocol::{
    ConMgrRegisterActorReq, ConMgrRegisterActorRsp, ConMgrRegisterActorStatus,
    CON_MGR_REGISTER_ACTOR_RSP_ID,
};
use crossbeam_channel::Sender;
use echo_requestee_protocol::echo_requestee_protocol;
use echo_requester_protocol::{
    echo_requester_protocol, EchoReq, EchoRsp, ECHO_REQ_ID, ECHO_RSP_ID,
};
use echo_start_complete_protocol::{
    echo_start_complete_protocol, EchoComplete, EchoStart, ECHO_START_ID,
};
use msg1::Msg1;
use msg2::Msg2;
use msg_header::{BoxMsgAny, MsgHeader};
use protocol::Protocol;
use protocol_set::ProtocolSet;
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
/// After instantiating the Controller issues an EchoStart with a ping_count.
/// The Client will then ping the partner with an EchoReq and expects
/// the partner to respond with an EchoRsp. After pinging the expected
/// number of times the Client will repspond to the Controller with
/// EchoDone.
///
/// Errors and not handled gracefully, this is just demo.
pub struct Client {
    pub name: String,
    pub actor_id: AnId,
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet,
    pub current_state: ProcessMsgFn<Self>,
    pub state_info_hash: StateInfoMap<Self>,
    pub partner_instance_id: Option<AnId>,
    pub partner_tx: Option<Sender<BoxMsgAny>>,
    pub controller_tx: Option<Sender<BoxMsgAny>>, // TODO: Change to instance_id and create a connection?
    pub ping_count: u64,
    connection: Connection,
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

    fn process_msg_any(&mut self, context: &dyn ActorContext, msg: BoxMsgAny) {
        (self.current_state)(self, context, msg);
    }

    fn their_bdlc_with_us(&self) -> actor_bi_dir_channel::BiDirLocalChannel {
        self.connection.their_bdlc_with_us.clone()
    }

    fn our_bdlc_with_them(&self) -> actor_bi_dir_channel::BiDirLocalChannel {
        self.connection.our_bdlc_with_them.clone()
    }

    fn connection(&self) -> Connection {
        self.connection.clone()
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
        let cip = cmd_init_protocol();
        let errp = echo_requester_protocol();
        let erep = echo_requestee_protocol();
        let escp = echo_start_complete_protocol();
        client_pm.insert(cip.id, cip.clone());
        client_pm.insert(errp.id, errp.clone());
        client_pm.insert(erep.id, erep.clone());
        client_pm.insert(escp.id, escp.clone());

        let client_ps = ProtocolSet::new("client_ps", CLIENT_PROTOCOL_SET_ID, client_pm);
        let mut this = Self {
            name: name.to_owned(),
            actor_id: CLIENT_ACTOR_ID,
            instance_id: AnId::new(),
            protocol_set: client_ps,
            current_state: Self::state0,
            state_info_hash: StateInfoMap::<Self>::new(),
            partner_instance_id: None,
            partner_tx: None,
            controller_tx: None,
            ping_count: 0,
            connection: Connection::new(),
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
            let req_msg = Box::new(EchoReq::new(&self.instance_id, counter));
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

    fn send_con_mgr_connect_req(&mut self, context: &dyn ActorContext, instance_id: &AnId) {
        let msg = Box::new(ConMgrConnectReq::new( &instance_id, context.their_bdlc_with_us()));
        println!("{}:send_con_mgr_req: instance_id={instance_id:?}", self.name);
        context.send_con_mgr(msg).unwrap();
    }

    pub fn state0(&mut self, context: &dyn ActorContext, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<EchoRsp>() {
            println!("{}:State0: {msg:?}", self.name);
            assert_eq!(msg.header.msg_id, ECHO_RSP_ID);
            self.send_echo_req_or_complete(msg.counter + 1);
        } else if let Some(msg) = msg_any.downcast_ref::<EchoReq>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.header.msg_id, ECHO_REQ_ID);
            let rsp_msg = Box::new(EchoRsp::new(&self.instance_id, msg.req_timestamp_ns, msg.counter));
            println!("{}:State0: sending rsp_msg={rsp_msg:?}", self.name);
            context.send_rsp(rsp_msg).unwrap();
        } else if let Some(msg) = msg_any.downcast_ref::<EchoStart>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.header.msg_id, ECHO_START_ID);
            if let Some(tx) = context.clone_rsp_tx() {
                self.partner_instance_id = Some(msg.partner_instance_id.clone());
                self.controller_tx = Some(tx);
                self.ping_count = msg.ping_count;
                self.send_con_mgr_connect_req(context, &msg.partner_instance_id);
            } else {
                println!(
                    "{}:State0: Error no controller_tx, can't start msg={msg:?}",
                    self.name
                );
            }
        } else if let Some(msg) = msg_any.downcast_ref::<ConMgrConnectRsp>() {
            println!("{}:State0: msg={msg:?}", self.name);
            assert_eq!(msg.header.msg_id, CON_MGR_CONNECT_RSP_ID);
            if msg.status == ConMgrConnectStatus::Success {
                println!("{}:State0: Successfully connected to partner start echoing", self.name);
                self.partner_tx = msg.partner_tx.clone();
                self.send_echo_req_or_complete(1);
            } else {
                println!("{}:State0: FAILED to connect to partner NO echoing, status: {:?}", self.name, msg.status);
            }
        } else if let Some(msg) = msg_any.downcast_ref::<Msg2>() {
            // Got a Msg2 so self send a Msg1 so our test passes :)
            println!("{}:State0: {msg:?}", self.name);
            let msg1 = Box::<Msg1>::default();

            //self.self_tx.as_ref().unwrap().send(msg1).unwrap();
            context.send_rsp(msg1).unwrap();
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
                context.their_bdlc_with_us(),
                context.actor_executor_tx(),
            ));
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
    use super::*;
    use actor_bi_dir_channel::{ActorBiDirChannel, BiDirLocalChannel};
    use chrono::Utc;
    use con_mgr_connect_protocol::CON_MGR_CONNECT_REQ_ID;
    use con_mgr_register_actor_protocol::{
        ConMgrRegisterActorRsp, ConMgrRegisterActorStatus, CON_MGR_REGISTER_ACTOR_REQ_ID,
    };
    use echo_start_complete_protocol::{EchoComplete, EchoStart, ECHO_COMPLETE_ID};
    use msg1::MSG1_ID;
    use msg_header::MsgHeader;

    struct Context {
        actor_executor_tx: Sender<BoxMsgAny>,
        con_mgr_tx: Sender<BoxMsgAny>,
        their_bdlc_with_us: BiDirLocalChannel,
        rsp_tx: Sender<BoxMsgAny>,
    }

    impl ActorContext for Context {
        fn actor_executor_tx(&self) -> &Sender<BoxMsgAny> {
            &self.actor_executor_tx
        }

        fn their_bdlc_with_us(&self) -> &BiDirLocalChannel {
            &self.their_bdlc_with_us
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

        fn clone_rsp_tx(&self) -> Option<Sender<BoxMsgAny>> {
            Some(self.rsp_tx.clone())
        }
    }

    #[test]
    fn test_cmd_init() {
        let supervisor_instance_id = AnId::new();

        let (client_bdlc, test_cmd_init_bdlc) = BiDirLocalChannel::new();

        // Both con_mgr_tx and rsp_tx are "this" test
        let client_context = Context {
            actor_executor_tx: test_cmd_init_bdlc.tx.clone(),
            con_mgr_tx: client_bdlc.tx.clone(),
            their_bdlc_with_us: test_cmd_init_bdlc.clone(),
            rsp_tx: client_bdlc.tx.clone(),
        };

        let mut client = Client::new("client");

        // First message must be CmdInit and client send
        let msg = Box::new(CmdInit::new());
        client.process_msg_any(&client_context, msg);

        // ConMgr is sent ConMgrRegisterActorReq and responds with
        // ConMgrRegisterActorRsp status: ConMgrRegisterActorStatus::Success
        let con_mgr_msg_any = test_cmd_init_bdlc.rx.recv().unwrap();
        let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&con_mgr_msg_any);
        assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
        let msg = Box::new(ConMgrRegisterActorRsp::new(
            &supervisor_instance_id,
            ConMgrRegisterActorStatus::Success,
        ));
        client.process_msg_any(&client_context, msg);

        // Send Msg2 expect Msg1 back
        let msg = Box::new(Msg2::new());
        client.process_msg_any(&client_context, msg);
        let recv_msg = test_cmd_init_bdlc.rx.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg),
            &MSG1_ID
        );
    }

    // Test various ping_counts including 0
    #[test]
    fn test_bi_dir_local_channel() {
        println!("test_bi_dir_local_channel:");
        let supervisor_instance_id = AnId::new();

        // BiDirLocalChannel between ctrl and clnt
        let (ctrl_with_clnt, clnt_with_ctrl) = BiDirLocalChannel::new();
        let ctrl_with_clnt_context = Context {
            actor_executor_tx: clnt_with_ctrl.tx.clone(),
            con_mgr_tx: clnt_with_ctrl.clone_tx(),
            their_bdlc_with_us: ctrl_with_clnt.clone(),
            rsp_tx: clnt_with_ctrl.clone_tx(),
        };

        // BiDirLocalChannel between clnt and srvr
        let (clnt_with_srvr, srvr_with_clnt) = BiDirLocalChannel::new();

        let mut client = Client::new("client");

        let test_instance_id = AnId::new();

        for ping_count in [0, 1, 5] {
            let clnt_with_ctrl_context = Context {
                actor_executor_tx: clnt_with_ctrl.tx.clone(),
                con_mgr_tx: clnt_with_ctrl.clone_tx(),
                their_bdlc_with_us: ctrl_with_clnt.clone(),
                rsp_tx: clnt_with_ctrl.clone_tx(),
            };

            let srvr_with_clnt_context = Context {
                actor_executor_tx: clnt_with_ctrl.tx.clone(),
                con_mgr_tx: clnt_with_ctrl.clone_tx(),
                their_bdlc_with_us: clnt_with_srvr.clone(),
                rsp_tx: srvr_with_clnt.clone_tx(),
            };

            // First message must be CmdInit and client will send a ConMsgRegisterActorReq
            let msg = Box::new(CmdInit::new());
            client.process_msg_any(&clnt_with_ctrl_context, msg);

            // ConMgr is sent ConMgrRegisterActorReq and responds with
            // ConMgrRegisterActorRsp status: ConMgrRegisterActorStatus::Success
            let con_mgr_msg_any = ctrl_with_clnt.recv().unwrap();
            let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&con_mgr_msg_any);
            assert_eq!(msg_id, &CON_MGR_REGISTER_ACTOR_REQ_ID);
            let msg = Box::new(ConMgrRegisterActorRsp::new(
                &supervisor_instance_id,
                ConMgrRegisterActorStatus::Success,
            ));
            client.process_msg_any(&ctrl_with_clnt_context, msg);

            // Controller sends EchoStart message to client
            println!("\ntest_bi_dir_local_channel: ping_count={ping_count}");
            let start_msg = Box::new(EchoStart::new(&test_instance_id, ping_count));
            ctrl_with_clnt.send(start_msg).unwrap();

            // Client receives EchoStart msg from control
            println!("test_bi_dir_local_channel: client process EchoStart");
            let start_msg_any = clnt_with_ctrl.recv().unwrap();
            client.process_msg_any(&clnt_with_ctrl_context, start_msg_any);

            // Client requests con_mgr to connect to partner
            println!("test_bi_dir_local_channel: client issues to con_mgr a ConMgrConnectReq");
            let connect_req_msg_any = ctrl_with_clnt.recv().unwrap();
            let connect_req_msg= connect_req_msg_any.downcast_ref::<ConMgrConnectReq>().unwrap();
            assert_eq!(&CON_MGR_CONNECT_REQ_ID, MsgHeader::get_msg_id_from_boxed_msg_any(&connect_req_msg_any));
            assert_eq!(connect_req_msg.instance_id, test_instance_id);

            // ConMgr forwards the request to the actors associated executor who connects the partner to the client
            // and the client receives the ConMgrConnectRsp
            println!("test_bi_dir_local_channel: client receives a ConMgrConnectRsp");
            let tx = clnt_with_srvr.clone_tx();
            let msg = Box::new(ConMgrConnectRsp::new(ConMgrConnectStatus::Success, Some(tx)));
            client.process_msg_any(&clnt_with_ctrl_context, msg);

            for i in 0..ping_count {
                println!(
                    "test_bi_dir_local_channel: server recv TOL {} of {ping_count}",
                    i + 1
                );

                // Server receives request message
                let req_msg_any = srvr_with_clnt.recv().unwrap();
                let req_msg = req_msg_any.downcast_ref::<EchoReq>().unwrap();
                println!("test_bi_dir_local_channel: received req_msg={req_msg:?}");

                // Server creates and sends rsp message
                let rsp_msg = Box::new(EchoRsp::new(&test_instance_id, Utc::now().timestamp_nanos(), req_msg.counter));
                srvr_with_clnt.send(rsp_msg).unwrap();

                // Client receives and processes rsp message from server
                let rsp_msg_any = clnt_with_srvr.recv().unwrap();
                client.process_msg_any(&srvr_with_clnt_context, rsp_msg_any);
            }

            // Controller receives Complete msg
            let complete_msg_any = ctrl_with_clnt.recv().unwrap();
            let complete_msg = complete_msg_any.downcast_ref::<EchoComplete>().unwrap();
            println!("test_bi_dir_local_channel: received complete msg={complete_msg:?}");
            assert_eq!(complete_msg.header.msg_id, ECHO_COMPLETE_ID);
        }
    }
}
