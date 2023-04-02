use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use actor::{Actor, ActorContext};
use actor_channel::{ActorChannel, ActorSender, VecActorChannel};

use actor_executor_protocol::actor_executor_protocol;
use an_id::{anid, paste, AnId};
use box_msg_any::BoxMsgAny;
use cmd_done::CmdDone;
use cmd_init_protocol::CmdInit;
use con_mgr_query_protocol::con_mgr_query_protocol;
use con_mgr_register_actor_protocol::con_mgr_register_actor_protocol;
use crossbeam_channel::Select;
use msg_header::MsgHeader;
use protocol::Protocol;
use protocol_set::ProtocolSet;
use req_add_actor::ReqAddActor;
use rsp_add_actor::RspAddActor;
use sender_map_by_instance_id::{sender_map_get, sender_map_insert};

#[allow(unused)]
#[derive(Debug)]
struct ActorExecutor {
    pub name: String,
    pub actor_id: AnId, // TODO: not used yet
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet, // TODO: not used yet
    pub vec_actor: Vec<Box<dyn Actor>>,
    pub vec_actor_chnl: VecActorChannel,
    con_mgr_instance_id: AnId,
    con_mgr_tx: ActorSender,
    done: bool,
}

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
        self.con_mgr_tx.send(msg)
    }

    fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        println!("ActorExecutor::send_self: Not implemented, just return Ok(())");
        Ok(())
    }

    fn send_rsp(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.rsp_tx.send(msg)
    }

    fn clone_rsp_tx(&self) -> Option<ActorSender> {
        Some(self.rsp_tx.clone())
    }
}

// From: https://www.uuidgenerator.net/version4
const ACTOR_EXECUTOR_ACTOR_ID: AnId = anid!("5c3d6e86-5e19-4ad8-a397-f446bedef1bd");
const ACTOR_EXECUTOR_PROTOCOL_SET_ID: AnId = anid!("09b50f0f-fb5d-4609-b657-0b1910d1d1dc");

#[allow(unused)]
impl ActorExecutor {
    // Returns a thread::JoinHandle and a Box<dyn ActorBiDirChannel> which
    // allows messages to be sent and received from the AeActor.
    pub fn start(name: &str, con_mgr_instance_id: &AnId) -> (JoinHandle<()>, ActorSender) {
        let ae_chnl = ActorChannel::new(name);

        // Convert name to string so it can be moved into the thread
        let name = name.to_string();

        // Create the ActorExecutor ProtocolSet.
        println!("AE:{}::start()", name);
        let mut pm = HashMap::<AnId, Protocol>::new();
        let ae_protocol = actor_executor_protocol();
        pm.insert(ae_protocol.id, ae_protocol.clone());
        let con_mgr_reg_actor_protocol = con_mgr_register_actor_protocol();
        pm.insert(
            con_mgr_reg_actor_protocol.id,
            con_mgr_reg_actor_protocol.clone(),
        );
        let query_protocol = con_mgr_query_protocol();
        pm.insert(query_protocol.id, query_protocol.clone());
        let ps_name = name.clone() + "_ps";
        let ps = ProtocolSet::new(&ps_name, ACTOR_EXECUTOR_PROTOCOL_SET_ID, pm);

        // these are moved
        let con_mgr_sender = sender_map_get(con_mgr_instance_id).unwrap();
        let cm_instance_id = *con_mgr_instance_id;
        let ae_chnl_sender = ae_chnl.sender.clone();

        let join_handle = thread::spawn(move || {
            let mut ae = Self {
                name: name.to_string(),
                actor_id: ACTOR_EXECUTOR_ACTOR_ID,
                instance_id: AnId::new(),
                protocol_set: ps,
                vec_actor: Vec::new(),
                vec_actor_chnl: VecActorChannel::new(),
                con_mgr_instance_id: cm_instance_id,
                con_mgr_tx: con_mgr_sender,
                done: false,
            };
            println!("AE:{}:+", ae.name);

            // Add our self to the sender map
            sender_map_insert(&ae.instance_id, &ae_chnl_sender);

            let mut selector = Select::new();
            let oper_idx = selector.recv(&ae_chnl.receiver.rx);
            assert_eq!(oper_idx, 0);

            while !ae.done {
                println!("AE:{}: TOL", ae.name);
                let oper = selector.select();
                let oper_idx = oper.index();

                if oper_idx == 0 {
                    // This message is for the AE itself
                    let result = oper.recv(&ae_chnl.receiver.rx);
                    match result {
                        Err(why) => {
                            // TODO: Error on our selves, is there anything else we need to do?
                            println!("AE:{}: error on recv: {why} `done = true`", ae.name);
                            ae.done = true;
                        }
                        Ok(msg_any) => {
                            // Got our message
                            println!("AE:{}: msg_any={msg_any:?}", ae.name);
                            if msg_any.downcast_ref::<ReqAddActor>().is_some() {
                                // It is a MsgReqAeAddActor, now downcast to concrete message so we can push it to vec_actor
                                let msg = msg_any.downcast::<ReqAddActor>().unwrap();
                                println!("AE:{}: msg={msg:?}", ae.name);

                                // Push actor
                                let actor_idx = ae.vec_actor.len();
                                ae.vec_actor.push(msg.actor);

                                // Push the actors channel
                                assert_eq!(ae.vec_actor_chnl.len(), actor_idx);
                                ae.vec_actor_chnl
                                    .push(ae.vec_actor[actor_idx].get_chnl().clone());

                                // Get a reference to the actors bdlcs
                                let chnl = ae.vec_actor_chnl.get(actor_idx);

                                // Add the actors receiver to the selector
                                println!("AE:{}: selector.recv(our_channel.get_recv())", ae.name);
                                selector.recv(&chnl.receiver.rx);

                                // Send the response message with their_channel
                                let msg_rsp = Box::new(RspAddActor::new(
                                    &ae.instance_id,
                                    ae.vec_actor[actor_idx].get_actor_id(),
                                    ae.vec_actor[actor_idx].get_instance_id(),
                                ));
                                println!("AE:{}: msg.rsp_tx.send msg={msg_rsp:?}", ae.name);
                                let rsp_tx = sender_map_get(&msg.header.src_id.unwrap()).unwrap();
                                rsp_tx.send(msg_rsp);

                                // Issue a CmdInit
                                let msg = Box::new(CmdInit::new(&ae.instance_id));
                                chnl.sender.send(msg).unwrap(); // TODO: Ignore error on release builds so we don't panic?

                                println!(
                                    "AE:{}: added new receiver for {}",
                                    ae.name,
                                    ae.vec_actor[actor_idx].get_name()
                                );
                            } else if let Some(msg) = msg_any.downcast_ref::<CmdDone>() {
                                println!("AE:{}: msg={msg:?}", ae.name);
                                ae.done = true;
                            } else {
                                println!("AE:{}: Uknown msg", ae.name);
                            }
                        }
                    }
                } else {
                    // This message for one of the actors running in the AE
                    let actor_idx = oper_idx - 1;
                    let actor = &mut ae.vec_actor[actor_idx];
                    println!(
                        "AE:{}: msg for vec_actor[{actor_idx}] {}",
                        ae.name,
                        actor.get_name(),
                    );
                    let chnl = ae.vec_actor_chnl.get(actor_idx);
                    if let Ok(msg_any) = oper.recv(&chnl.receiver.rx).map_err(|why| {
                        // TODO: What should we do here?
                        panic!("AE:{}: {} error on recv: {why}", ae.name, actor.get_name())
                    }) {
                        println!(
                            "AE:{}: call process_msg_any[{actor_idx}] {} msg_id={}",
                            ae.name,
                            actor.get_name(),
                            MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any),
                        );
                        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(&msg_any);
                        let rsp_tx = match src_id {
                            Some(src_id) => {
                                if let Some(sender) = sender_map_get(src_id) {
                                    sender.clone()
                                } else {
                                    let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
                                    panic!(
                                        "AE:{}: BUG; msg_any has msg_id={msg_id} and src_id={src_id:?} but not in sender_map",
                                        ae.name);
                                }
                            }
                            None => {
                                let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
                                panic!(
                                    "AE:{}: There is no src_id in msg_any header.msg_id={msg_id:?}",
                                    ae.name
                                );
                            }
                        };
                        let context = Context {
                            // TODO: All this cloning for each msg is slow, we need an array/hash of these Context's
                            actor_executor_tx: ae_chnl_sender.clone(),
                            con_mgr_tx: ae.con_mgr_tx.clone(),
                            rsp_tx,
                        };
                        actor.process_msg_any(&context, msg_any);
                        println!(
                            "AE:{}: retf process_msg_any[{actor_idx}] {}",
                            ae.name,
                            actor.get_name(),
                        );
                        if actor.done() {
                            panic!(
                                "AE:{}: {} reported done, what to do?",
                                ae.name,
                                actor.get_name()
                            );
                        }
                    }
                }
            }

            // TODO: Should we be cleaning things up, like telling the Manager?
            println!("AE:{}:-", ae.name);
        });

        (join_handle, ae_chnl.sender)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use actor_channel::ActorChannel;
    use an_id::AnId;
    use client::Client;
    use cmd_done::CmdDone;
    use con_mgr::ConMgr;
    use echo_requestee_protocol::{EchoReq, EchoRsp};
    use echo_start_complete_protocol::{EchoComplete, EchoStart};
    //use box_msg_any::BoxMsgAny;
    use server::Server;

    use super::*;

    // Initialize create supervisor_id, and supervisor_chnl, ConMg and ActorExecutor
    // starting ActorExecutor and adding ConMgr to it
    fn initialize() -> (AnId, ActorChannel, ActorSender, JoinHandle<()>) {
        // Add supervisor to sender_map
        let supervisor_instance_id = AnId::new();
        let supervisor_chnl = ActorChannel::new("supervisor");
        sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

        // Create connection manager
        let con_mgr_name = "con_mgr";
        let con_mgr = Box::new(ConMgr::new(con_mgr_name));

        // Start an ActorExecutor
        let (ae_join_handle, ae_sender) = ActorExecutor::start("ae", con_mgr.get_instance_id());
        println!("test_con_mgr_server: ae_sender={ae_sender:?}");

        // Add con_mgr to ActorExecutor
        let msg = Box::new(ReqAddActor::new(&supervisor_instance_id, con_mgr));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_server: sent {} to ae", con_mgr_name);
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_server: recvd rsp_add_actor={msg:?}");

        (
            supervisor_instance_id,
            supervisor_chnl,
            ae_sender,
            ae_join_handle,
        )
    }

    #[test]
    fn test_con_mgr_server() {
        println!("\ntest_con_mgr_server:+");

        let (supervisor_instance_id, supervisor_chnl, ae_sender, ae_join_handle) = initialize();

        // Add Server s1 to ActorExecutor
        let s1_name = "server1";
        let s1 = Box::new(Server::new(s1_name));
        let s1_instance_id = s1.get_instance_id().clone();
        println!("test_con_mgr_server: create s1={s1:?}");
        let msg = Box::new(ReqAddActor::new(&supervisor_instance_id, s1));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_server: sent {} to ae", s1_name);
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_server: recvd rsp_add_actor={msg:?}");

        println!("test_con_mgr_server: send EchoReq");
        sender_map_get(&s1_instance_id)
            .unwrap()
            .send(Box::new(EchoReq::new(&supervisor_instance_id, 1)))
            .unwrap();
        println!("test_con_mgr_server: sent EchoReq");

        println!("test_con_mgr_server: wait EchoRsp");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoRsp>().unwrap();
        println!("test_con_mgr_server: recv EchoRsp={msg_rsp:?}");
        assert_eq!(msg_rsp.counter, 1);

        println!("test_con_mgr_server: send CmdDone");
        let msg = Box::new(CmdDone::new(&supervisor_instance_id));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_server: sent CmdDone");

        println!("test_con_mgr_server: join ae to complete");
        ae_join_handle.join().unwrap();
        println!("test_con_mgr_server: join ae to completed");

        println!("test_con_mgr_server:-");
    }

    #[test]
    fn test_con_mgr_client_server() {
        println!("\ntest_con_mgr_client_server:+");

        let (supervisor_instance_id, supervisor_chnl, ae_sender, ae_join_handle) = initialize();

        // Add client1 to ActorExecutor
        let c1_name = "client1";
        let c1 = Box::new(Client::new(c1_name));
        let c1_instance_id = *c1.get_instance_id();
        println!("test_con_mgr_client_server: {c1_instance_id} create c1={c1:?}");
        let msg = Box::new(ReqAddActor::new(&supervisor_instance_id, c1));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent {} to ae", c1_name);
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_client_server: recvd rsp_add_actor={msg:?}");

        // Add server1 to ActorExecutor
        let s1_name = "server1";
        let s1 = Box::new(Server::new(s1_name));
        let s1_instance_id = *s1.get_instance_id();
        println!("test_con_mgr_client_server: create s1={s1:?}");
        let msg = Box::new(ReqAddActor::new(&supervisor_instance_id, s1));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent {} to ae", s1_name);
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_client_server: recvd rsp_add_actor={msg:?}");

        // Send EchoStart to c1
        println!("test_con_mgr_client_server: send EchoStart");
        let c1_tx = match sender_map_get(&c1_instance_id) {
            Some(tx) => tx,
            None => {
                println!("test_con_mgr_client_server: c1_tx not found");
                panic!();
            }
        };
        match c1_tx.send(Box::new(EchoStart::new(
            &supervisor_instance_id,
            &s1_instance_id,
            10,
        ))) {
            Ok(_) => {}
            Err(e) => {
                println!("test_con_mgr_client_server: send EchoStart failed e={e:?}");
                panic!();
            }
        }
        println!("test_con_mgr_client_server: sent EchoStart");

        // Wait for EchoComplete from c1
        println!("test_con_mgr_client_server: wait EchoComplete");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoComplete>().unwrap();
        println!("test_con_mgr_client_server: recv EchoComplete={msg_rsp:?}");

        println!("test_con_mgr_client_server: send CmdDone");
        let msg = Box::new(CmdDone::new(&supervisor_instance_id));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent CmdDone");

        println!("test_con_mgr_client_server: join ae to complete");
        ae_join_handle.join().unwrap();
        println!("test_con_mgr_client_server: join ae to completed");

        //drop(supervisor_tx);
        //drop(supervisor_rx);

        println!("test_con_mgr_client_server:-");
    }
}
