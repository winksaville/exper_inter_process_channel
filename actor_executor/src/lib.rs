use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use actor::{Actor, ActorContext};
use actor_channel::{ActorChannel, ActorReceiver, ActorSender, VecActorChannel};

use actor_executor_protocol::actor_executor_protocol;
use an_id::{anid, paste, AnId};
use box_msg_any::BoxMsgAny;
use cmd_done_issuee_protocol::{cmd_done_issuee_protocol, CmdDone};
use cmd_init_issuer_protocol::{cmd_init_issuer_protocol, CmdInit};
use con_mgr::ConMgr;
use con_mgr_query_protocol::con_mgr_query_protocol;
use con_mgr_register_actor_protocol::con_mgr_register_actor_protocol;
use crossbeam_channel::Select;
use msg_header::MsgHeader;
use protocol::Protocol;
use protocol_set::ProtocolSet;
use req_add_actor::ReqAddActor;
use rsp_add_actor::RspAddActor;
use sender_map_by_instance_id::{sender_map_get, sender_map_insert};

// Helper functions for setting up a cluster local of actors for testing.
// Someday something like this will be use in "production", but for now
// this is for testing only!
//
// Here is an example of how to use this:
//
// // Initialize Supervisor starting a single ActorExecutor and the connection manager
// let (
//     supervisor_instance_id,
//     supervisor_chnl,
//     ae_join_handle,
//     ae_instance_id,
//     con_mgr_instance_id,
// ) = initialize_supervisor_con_mgr_actor_executor_blocking();
// let ae_sender = sender_map_get(&ae_instance_id).unwrap();
//
// // Start a second ActorExecutor
// let (ae2_join_handle, ae2_instance_id) = ActorExecutor::start("ae2", &con_mgr_instance_id);
// let ae2_sender = sender_map_get(&ae2_instance_id).unwrap();
// println!("test_con_mgr_server: ae_sender={ae_sender:?}");
//
// // Add client1 to ActorExecutor
// let c1 = Box::new(Client::new("client1"));
// let (_c1_actor_id, c1_instance_id) = add_actor_to_actor_executor_blocking(
//     c1,
//     &ae_instance_id,
//     &supervisor_instance_id,
//     &supervisor_chnl.receiver,
// );
//
// // Add server1 to ActorExecutor
// let s1 = Box::new(Server::new("server1"));
// let (_s1_actor_id, s1_instance_id) = add_actor_to_actor_executor_blocking(
//     s1,
//     &ae2_instance_id,
//     &supervisor_instance_id,
//     &supervisor_chnl.receiver,
// );

// Add an actor to the ActorExecutor blocking until the actor is added
pub fn add_actor_to_actor_executor_blocking(
    actor_boxed: Box<dyn Actor>,
    ae_instance_id: &AnId,
    supervisor_instance_id: &AnId,
    supervisor_receiver: &ActorReceiver,
) -> (AnId, AnId) {
    let msg = Box::new(ReqAddActor::new(
        ae_instance_id,
        supervisor_instance_id,
        actor_boxed,
    ));
    let ae_sender = sender_map_get(ae_instance_id).unwrap();
    ae_sender.send(msg).unwrap();
    let msg_any = supervisor_receiver.recv().unwrap();
    let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();

    (msg.actor_id, msg.actor_instance_id)
}

// Initialize create supervisor_id, and supervisor_chnl, ConMg and ActorExecutor
// starting ActorExecutor and adding ConMgr to it.
//
// Returns supervisor_instance_id, supervisor_chnl, ae_join_handle, ae_instance_id, con_mgr_instance_id
pub fn initialize_supervisor_con_mgr_actor_executor_blocking(
) -> (AnId, ActorChannel, JoinHandle<()>, AnId, AnId) {
    // Add supervisor to sender_map
    let supervisor_instance_id = AnId::new();
    let supervisor_chnl = ActorChannel::new("supervisor", &supervisor_instance_id);
    sender_map_insert(&supervisor_instance_id, &supervisor_chnl.sender);

    // Create connection manager
    let con_mgr_name = "con_mgr";
    let con_mgr = Box::new(ConMgr::new(con_mgr_name));
    let con_mgr_instance_id = *con_mgr.get_instance_id();

    // Start an ActorExecutor
    let (ae_join_handle, ae_instance_id) = ActorExecutor::start("ae", &con_mgr_instance_id);
    println!("test_con_mgr_server: ae_instance_id={ae_instance_id:?}");

    // Add con_mgr to ActorExecutor
    let msg = Box::new(ReqAddActor::new(
        &ae_instance_id,
        &supervisor_instance_id,
        con_mgr,
    ));
    let ae_sender = sender_map_get(&ae_instance_id).unwrap();
    ae_sender.send(msg).unwrap();
    println!("test_con_mgr_server: sent {} to ae", con_mgr_name);
    let msg_any = supervisor_chnl.receiver.recv().unwrap();
    let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
    println!("test_con_mgr_server: recvd rsp_add_actor={msg:?}");

    (
        supervisor_instance_id,
        supervisor_chnl,
        ae_join_handle,
        ae_instance_id,
        con_mgr_instance_id,
    )
}

#[allow(unused)]
#[derive(Debug)]
pub struct ActorExecutor {
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
    ae_sndr: ActorSender,
    con_mgr_sndr: ActorSender,
    dst_sndr: ActorSender,
}

impl ActorContext for Context {
    fn actor_executor_sndr(&self) -> &ActorSender {
        &self.ae_sndr
    }

    fn send_con_mgr(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.con_mgr_sndr.send(msg)
    }

    fn get_con_mgr_instance_id(&self) -> &AnId {
        self.con_mgr_sndr.get_dst_instance_id()
    }

    fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        println!("ActorExecutor::send_self: Not implemented, just return Ok(())");
        Ok(())
    }

    fn send_dst(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.dst_sndr.send(msg)
    }

    fn get_dst_instance_id(&self) -> &AnId {
        self.dst_sndr.get_dst_instance_id()
    }

    fn clone_dst_sndr(&self) -> ActorSender {
        self.dst_sndr.clone()
    }
}

// From: https://www.uuidgenerator.net/version4
const ACTOR_EXECUTOR_ACTOR_ID: AnId = anid!("5c3d6e86-5e19-4ad8-a397-f446bedef1bd");
const ACTOR_EXECUTOR_PROTOCOL_SET_ID: AnId = anid!("09b50f0f-fb5d-4609-b657-0b1910d1d1dc");

#[allow(unused)]
impl ActorExecutor {
    // Returns a thread::JoinHandle and a Box<dyn ActorBiDirChannel> which
    // allows messages to be sent and received from the AeActor.
    //
    // Returns the ActorExecutor join handle and its instance_id
    pub fn start(name: &str, con_mgr_instance_id: &AnId) -> (JoinHandle<()>, AnId) {
        let ae_iid = AnId::new();
        let ae_chnl = ActorChannel::new(name, &ae_iid);
        sender_map_insert(&ae_iid, &ae_chnl.sender);

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
        let ci_irp = cmd_init_issuer_protocol();
        pm.insert(ci_irp.id, ci_irp.clone());
        let cd_iep = cmd_done_issuee_protocol();
        pm.insert(cd_iep.id, cd_iep.clone());

        let ps_name = name.clone() + "_ps";
        let ps = ProtocolSet::new(&ps_name, ACTOR_EXECUTOR_PROTOCOL_SET_ID, pm);

        // these are moved
        let con_mgr_sender = sender_map_get(con_mgr_instance_id).unwrap();
        let cm_instance_id = *con_mgr_instance_id;
        let ae_instance_id = ae_iid;

        let join_handle = thread::spawn(move || {
            let mut ae = Self {
                name: name.to_string(),
                actor_id: ACTOR_EXECUTOR_ACTOR_ID,
                instance_id: ae_instance_id,
                protocol_set: ps,
                vec_actor: Vec::new(),
                vec_actor_chnl: VecActorChannel::new(),
                con_mgr_instance_id: cm_instance_id,
                con_mgr_tx: con_mgr_sender,
                done: false,
            };
            println!("AE:{}:+", ae.name);

            let mut selector = Select::new();
            let oper_idx = selector.recv(&ae_chnl.receiver.rx);
            assert_eq!(oper_idx, 0);

            while !ae.done {
                println!("AE:{}: TOL", ae.name);
                let oper = selector.select();
                let oper_idx = oper.index();

                if oper_idx == 0 {
                    println!("AE:{}:self: msg received", ae.name);
                    // This message is for the AE itself
                    let result = oper.recv(&ae_chnl.receiver.rx);
                    match result {
                        Err(why) => {
                            // TODO: Error on our selves, is there anything else we need to do?
                            println!("AE:{}:self: error on recv: {why} `done = true`", ae.name);
                            ae.done = true;
                        }
                        Ok(msg_any) => {
                            // Got our message
                            println!("AE:{}:self: msg_any={msg_any:?}", ae.name);
                            if msg_any.downcast_ref::<ReqAddActor>().is_some() {
                                // It is a MsgReqAeAddActor, now downcast to concrete message so we can push it to vec_actor
                                let msg = msg_any.downcast::<ReqAddActor>().unwrap();
                                println!("AE:{}:self: msg={msg:?}", ae.name);

                                // Get the destination id of the actor requesting the registration
                                let requester_dst_id = *msg.src_id();

                                // Get the actor's instance_id that is being registered
                                let actor_instance_id = *msg.actor.get_instance_id();

                                // Push actor
                                let actor_idx = ae.vec_actor.len();
                                ae.vec_actor.push(msg.actor);

                                // Push the actors channel
                                assert_eq!(ae.vec_actor_chnl.len(), actor_idx);
                                ae.vec_actor_chnl
                                    .push(ae.vec_actor[actor_idx].get_chnl().clone());

                                // Get a reference to the actors channel
                                let chnl = ae.vec_actor_chnl.get(actor_idx);

                                // Add the actors receiver to the selector
                                println!(
                                    "AE:{}:self: selector.recv(our_channel.get_recv())",
                                    ae.name
                                );
                                selector.recv(&chnl.receiver.rx);

                                // Send the response message with their instance_id
                                let sndr = sender_map_get(&requester_dst_id).unwrap();
                                let msg_rsp = Box::new(RspAddActor::new(
                                    sndr.get_dst_instance_id(),
                                    &ae.instance_id,
                                    ae.vec_actor[actor_idx].get_actor_id(),
                                    &actor_instance_id,
                                ));
                                println!("AE:{}:self: respond with msg={msg_rsp:?}", ae.name);
                                println!("AE:{}:self: sender={sndr:?}", ae.name);
                                sndr.send(msg_rsp);

                                // Issue a CmdInit
                                let msg =
                                    Box::new(CmdInit::new(&actor_instance_id, &ae.instance_id));
                                chnl.sender.send(msg).unwrap(); // TODO: Ignore error on release builds so we don't panic?

                                println!(
                                    "AE:{}:self: added new receiver for {}",
                                    ae.name,
                                    ae.vec_actor[actor_idx].get_name()
                                );
                            } else if let Some(msg) = msg_any.downcast_ref::<CmdDone>() {
                                println!("AE:{}:self: msg={msg:?}", ae.name);
                                ae.done = true;
                            } else {
                                println!(
                                    "AE:{}:self: Uknown msg_id={:?}",
                                    ae.name,
                                    MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any)
                                );
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
                        let rsp_tx = if let Some(sender) = sender_map_get(src_id) {
                            sender.clone()
                        } else {
                            let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any);
                            panic!(
                                "AE:{}: BUG; send_map_get returned None for src_id={src_id:?} with msg_id={msg_id:?}",
                                ae.name);
                        };
                        let context = Context {
                            // TODO: All this cloning for each msg is slow, we need an array/hash of these Context's
                            ae_sndr: ae_chnl.sender.clone(),
                            con_mgr_sndr: ae.con_mgr_tx.clone(),
                            dst_sndr: rsp_tx,
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

        (join_handle, ae_iid)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use client::Client;
    use cmd_done_issuer_protocol::CmdDone;
    use echo_requestee_protocol::{EchoReq, EchoRsp};
    use echo_start_complete_protocol::{EchoComplete, EchoStart};
    use sender_map_by_instance_id::sender_map_get;
    use server::Server;

    #[test]
    fn test_con_mgr_server() {
        println!("\ntest_con_mgr_server:+");

        let (supervisor_instance_id, supervisor_chnl, ae_join_handle, ae_instance_id, _) =
            initialize_supervisor_con_mgr_actor_executor_blocking();
        let ae_sender = sender_map_get(&ae_instance_id).unwrap();

        // Add Server s1 to ActorExecutor
        let s1 = Box::new(Server::new("server1"));
        let (_s1_actor_id, s1_instance_id) = add_actor_to_actor_executor_blocking(
            s1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        println!("test_con_mgr_server: send EchoReq");
        sender_map_get(&s1_instance_id)
            .unwrap()
            .send(Box::new(EchoReq::new(
                &s1_instance_id,
                &supervisor_instance_id,
                1,
            )))
            .unwrap();
        println!("test_con_mgr_server: sent EchoReq");

        println!("test_con_mgr_server: wait EchoRsp");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoRsp>().unwrap();
        println!("test_con_mgr_server: recv EchoRsp={msg_rsp:?}");
        assert_eq!(msg_rsp.counter, 1);

        println!("test_con_mgr_server: send CmdDone");
        let msg = Box::new(CmdDone::new(msg_rsp.dst_id(), &supervisor_instance_id));
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

        let (supervisor_instance_id, supervisor_chnl, ae_join_handle, ae_instance_id, _) =
            initialize_supervisor_con_mgr_actor_executor_blocking();
        let ae_sender = sender_map_get(&ae_instance_id).unwrap();

        // Add client1 to ActorExecutor
        let c1 = Box::new(Client::new("client1"));
        let (_c1_actor_id, c1_instance_id) = add_actor_to_actor_executor_blocking(
            c1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Add server1 to ActorExecutor
        let s1 = Box::new(Server::new("server1"));
        let (_s1_actor_id, s1_instance_id) = add_actor_to_actor_executor_blocking(
            s1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Send EchoStart to c1
        println!("test_con_mgr_client_server: send EchoStart");
        let c1_sndr = match sender_map_get(&c1_instance_id) {
            Some(tx) => tx,
            None => {
                println!("test_con_mgr_client_server: c1_tx not found");
                panic!();
            }
        };
        match c1_sndr.send(Box::new(EchoStart::new(
            c1_sndr.get_dst_instance_id(),
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
        let msg = Box::new(CmdDone::new(&ae_instance_id, &supervisor_instance_id));
        ae_sender.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent CmdDone");

        println!("test_con_mgr_client_server: join ae to complete");
        ae_join_handle.join().unwrap();
        println!("test_con_mgr_client_server: join ae to completed");

        //drop(supervisor_tx);
        //drop(supervisor_rx);

        println!("test_con_mgr_client_server:-");
    }

    #[test]
    fn test_multiple_ae() {
        println!("\ntest_multiple_ae:+");

        // Initialize Supervisor starting a single ActorExecutor and the connection manager
        let (
            supervisor_instance_id,
            supervisor_chnl,
            ae_join_handle,
            ae_instance_id,
            con_mgr_instance_id,
        ) = initialize_supervisor_con_mgr_actor_executor_blocking();
        let ae_sender = sender_map_get(&ae_instance_id).unwrap();

        // Start a second ActorExecutor
        let (ae2_join_handle, ae2_instance_id) = ActorExecutor::start("ae2", &con_mgr_instance_id);
        let ae2_sender = sender_map_get(&ae2_instance_id).unwrap();
        println!("test_con_mgr_server: ae_sender={ae_sender:?}");

        // Add client1 to ActorExecutor
        let c1 = Box::new(Client::new("client1"));
        let (_c1_actor_id, c1_instance_id) = add_actor_to_actor_executor_blocking(
            c1,
            &ae_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Add server1 to ActorExecutor
        let s1 = Box::new(Server::new("server1"));
        let (_s1_actor_id, s1_instance_id) = add_actor_to_actor_executor_blocking(
            s1,
            &ae2_instance_id,
            &supervisor_instance_id,
            &supervisor_chnl.receiver,
        );

        // Send EchoStart to c1
        println!("test_multiple_ae: send EchoStart");
        let c1_tx = match sender_map_get(&c1_instance_id) {
            Some(tx) => tx,
            None => {
                println!("test_multiple_ae: c1_tx not found");
                panic!();
            }
        };
        match c1_tx.send(Box::new(EchoStart::new(
            &c1_tx.get_dst_instance_id(),
            &supervisor_instance_id,
            &s1_instance_id,
            10,
        ))) {
            Ok(_) => {}
            Err(e) => {
                println!("test_multiple_ae: send EchoStart failed e={e:?}");
                panic!();
            }
        }
        println!("test_multiple_ae: sent EchoStart");

        // Wait for EchoComplete from c1
        println!("test_multiple_ae: wait EchoComplete");
        let msg_any = supervisor_chnl.receiver.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoComplete>().unwrap();
        println!("test_multiple_ae: recv EchoComplete={msg_rsp:?}");

        println!("test_multiple_ae: send CmdDone to ae");
        let msg = Box::new(CmdDone::new(&ae_instance_id, &supervisor_instance_id));
        ae_sender.send(msg).unwrap();
        println!("test_multiple_ae: sent ae CmdDone");

        println!("test_multiple_ae: join ae");
        ae_join_handle.join().unwrap();
        println!("test_multiple_ae: join ae has completed");

        println!("test_multiple_ae: send CmdDone to ae2");
        let msg = Box::new(CmdDone::new(&ae2_instance_id, &supervisor_instance_id));
        ae2_sender.send(msg).unwrap();
        println!("test_multiple_ae: sent ae2 CmdDone");

        println!("test_multiple_ae: join ae2");
        ae2_join_handle.join().unwrap();
        println!("test_multiple_ae: join ae2 has completed");

        println!("test_multiple_ae:-");
    }
}
