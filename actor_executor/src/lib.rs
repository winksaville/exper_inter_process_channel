use std::{thread::{self, JoinHandle}, collections::HashMap};

use actor::{Actor, ActorContext};
use actor_bi_dir_channel::{ActorBiDirChannel, BiDirLocalChannel, Connection, VecConnection};

use an_id::{AnId, paste, anid};
use cmd_done::CmdDone;
use cmd_init_protocol::CmdInit;
use actor_executor_protocol::actor_executor_protocol;
use con_mgr_connect_protocol::con_mgr_connect_protocol;
use con_mgr_register_actor_protocol::con_mgr_register_actor_protocol;
use con_mgr::rsp_tx_map_get;
use crossbeam_channel::{Select, Sender};
use msg_header::{BoxMsgAny, MsgHeader};
use protocol::Protocol;
use protocol_set::ProtocolSet;
use req_add_actor::ReqAddActor;
use req_their_bi_dir_channel::ReqTheirBiDirChannel;
use rsp_add_actor::RspAddActor;
use rsp_their_bi_dir_channel::RspTheirBiDirChannel;

#[allow(unused)]
#[derive(Debug)]
struct ActorExecutor {
    pub name: String,
    pub actor_id: AnId, // TODO: not used yet
    pub instance_id: AnId,
    pub protocol_set: ProtocolSet, // TODO: not used yet
    pub actor_vec: Vec<Box<dyn Actor>>,
    pub bi_dir_channels_vec: VecConnection,
    con_mgr_bdlc: BiDirLocalChannel,
    done: bool,
}

struct Context {
    actor_executor_tx: Sender<BoxMsgAny>,
    con_mgr_tx: Sender<BoxMsgAny>,
    their_bdlc_with_us: BiDirLocalChannel,
    rsp_tx: Option<Sender<BoxMsgAny>>,
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
        println!("ActorExecutor::send_self: Not implemented, just return Ok(())");
        Ok(())
    }

    fn send_rsp(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        match &self.rsp_tx {
            Some(rsp_tx) => {
                Ok(rsp_tx.send(msg)?)
            }
            None => {
                println!("ActorExecutor::send_rsp: No response channel");
                Err("None".into())
            }
        }
    }

    fn clone_rsp_tx(&self) -> Option<Sender<BoxMsgAny>> {
        match &self.rsp_tx {
            Some(rsp_tx) => {
                Some(rsp_tx.clone())
            }
            None => {
                println!("ActorExecutor::clone_rsp_tx: No response channel");
                None
            }
        }
    }
}

// From: https://www.uuidgenerator.net/version4
const ACTOR_EXECUTOR_ACTOR_ID: AnId = anid!("5c3d6e86-5e19-4ad8-a397-f446bedef1bd");
const ACTOR_EXECUTOR_PROTOCOL_SET_ID: AnId = anid!("09b50f0f-fb5d-4609-b657-0b1910d1d1dc");

#[allow(unused)]
impl ActorExecutor {
    // Returns a thread::JoinHandle and a Box<dyn ActorBiDirChannel> which
    // allows messages to be sent and received from the AeActor.
    pub fn start(
        name: &str,
        con_mgr_bdlc: BiDirLocalChannel,
    ) -> (JoinHandle<()>, Box<BiDirLocalChannel>) {
        let ae_actor_bi_dir_channels = Connection::new();
        let their_bdlc_with_us = Box::new(ae_actor_bi_dir_channels.their_bdlc_with_us.clone());

        // Convert name to string so it can be moved into the thread
        let name = name.to_string();

        // Create the ConMgr ProtocolSet.
        println!("ConMgr::new({})", name);
        let mut pm = HashMap::<AnId, Protocol>::new();
        let ae_protocol = actor_executor_protocol();
        pm.insert(ae_protocol.id, ae_protocol.clone());
        let con_mgr_reg_actor_protocol = con_mgr_register_actor_protocol();
        pm.insert(
            con_mgr_reg_actor_protocol.id,
            con_mgr_reg_actor_protocol.clone(),
        );
        let connnect_protocol = con_mgr_connect_protocol();
        pm.insert(connnect_protocol.id, connnect_protocol.clone());
        let ps_name = name.clone() + "_ps";
        let ps = ProtocolSet::new(&ps_name, ACTOR_EXECUTOR_PROTOCOL_SET_ID, pm);

        let join_handle = thread::spawn(move || {
            let mut ae = Self {
                name: name.to_string(),
                actor_id: ACTOR_EXECUTOR_ACTOR_ID,
                instance_id: AnId::new(),
                protocol_set: ps,
                actor_vec: Vec::new(),
                bi_dir_channels_vec: VecConnection::new(),
                con_mgr_bdlc,
                done: false,
            };
            println!("AE:{}:+", ae.name);

            let mut selector = Select::new();
            let oper_idx = selector.recv(ae_actor_bi_dir_channels.our_bdlc_with_them.get_recv());
            assert_eq!(oper_idx, 0);

            while !ae.done {
                println!("AE:{}: TOL", ae.name);
                let oper = selector.select();
                let oper_idx = oper.index();

                if oper_idx == 0 {
                    // This messageis for the AE itself
                    let rx = ae_actor_bi_dir_channels.our_bdlc_with_them.get_recv();

                    let result = oper.recv(rx);
                    match result {
                        Err(why) => {
                            // TODO: Error on our selves make done, is there anything else we need to do?
                            println!("AE:{}: error on recv: {why} `done = true`", ae.name);
                            ae.done = true;
                        }
                        Ok(msg_any) => {
                            // This is a message for this ActorExecutor!!!
                            println!("AE:{}: msg_any={msg_any:?}", ae.name);
                            if msg_any.downcast_ref::<ReqAddActor>().is_some() {
                                // It is a MsgReqAeAddActor, now downcast to concrete message so we can push it to actor_vec
                                let msg = msg_any.downcast::<ReqAddActor>().unwrap();
                                println!("AE:{}: msg={msg:?}", ae.name);

                                // Push actor
                                let actor_idx = ae.actor_vec.len();
                                ae.actor_vec.push(msg.actor);

                                // Push the actors bdlcs
                                assert_eq!(ae.bi_dir_channels_vec.len(), actor_idx);
                                ae.bi_dir_channels_vec
                                    .push(ae.actor_vec[actor_idx].connection());

                                // Get a reference to the actors bdlcs
                                let bdlcs = ae.bi_dir_channels_vec.get(actor_idx);

                                println!("AE:{}: selector.recv(our_channel.get_recv())", ae.name);
                                selector.recv(bdlcs.our_bdlc_with_them.get_recv());

                                // Send the response message with their_channel
                                let msg_rsp = Box::new(RspAddActor::new(
                                    &ae.instance_id, // TODO: the ActorExecutor and it's instance_id need to be registered with the ConMgr
                                    Box::new(bdlcs.their_bdlc_with_us.clone())
                                ));
                                println!("AE:{}: msg.rsp_tx.send msg={msg_rsp:?}", ae.name);
                                let rsp_tx = rsp_tx_map_get(&msg.header.src_id.unwrap()).unwrap();
                                rsp_tx.send(msg_rsp);

                                // Issue a CmdInit
                                let msg = Box::new(CmdInit::new());
                                bdlcs.our_bdlc_with_them.self_tx.send(msg).unwrap(); // TODO: Ignore error on release builds so we don't panic?

                                println!(
                                    "AE:{}: added new receiver for {}",
                                    ae.name,
                                    ae.actor_vec[actor_idx].get_name()
                                );
                            } else if let Some(msg) = msg_any.downcast_ref::<CmdDone>() {
                                println!("AE:{}: msg={msg:?}", ae.name);
                                ae.done = true;
                            } else if let Some(msg) = msg_any.downcast_ref::<ReqTheirBiDirChannel>()
                            {
                                println!("AE:{}: msg={msg:?}", ae.name);
                                let connection = ae.bi_dir_channels_vec.get(msg.handle);
                                let their_bdlc_with_us = connection.their_bdlc_with_us.clone();
                                let msg_rsp = Box::new(RspTheirBiDirChannel::new(Box::new(
                                    their_bdlc_with_us,
                                )));

                                // send msg_rsp
                                println!("AE:{}: send msg_rsp={msg_rsp:?}", ae.name);
                                msg.rsp_tx.send(msg_rsp).unwrap();
                            } else {
                                println!("AE:{}: Uknown msg", ae.name);
                            }
                        }
                    }
                } else {
                    // This message for one of the actors running in the AE
                    let actor_idx = oper_idx - 1;
                    let actor = &mut ae.actor_vec[actor_idx];
                    println!(
                        "AE:{}: msg for actor_vec[{actor_idx}] {}",
                        ae.name,
                        actor.get_name(),
                    );
                    let bdlcs = ae.bi_dir_channels_vec.get(actor_idx);
                    let rx = bdlcs.our_bdlc_with_them.get_recv();
                    if let Ok(msg_any) = oper.recv(rx).map_err(|why| {
                        // TODO: What should we do here?
                        panic!("AE:{}: {} error on recv: {why}", ae.name, actor.get_name())
                    }) {
                        println!(
                            "AE:{}: call process_msg_any[{actor_idx}] {} msg_id={}",
                            ae.name,
                            actor.get_name(),
                            MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any),
                        );
                        let cm = ae.con_mgr_bdlc.clone();
                        let rsp_tx = match MsgHeader::get_src_id_from_boxed_msg_any(&msg_any) {
                            Some(src_id) => {
                                con_mgr::rsp_tx_map_get(src_id)
                            }
                            None => {
                                println!("AE:{}: There is no src_id in msg_any header.msg_id={:?}", ae.name, MsgHeader::get_msg_id_from_boxed_msg_any(&msg_any));
                                None
                            }
                        };
                        let context = Context {
                            // TODO: All this cloning for each msg is slow, we need an array/hash of these Context's
                            actor_executor_tx: ae_actor_bi_dir_channels
                                .their_bdlc_with_us
                                .tx
                                .clone(),
                            con_mgr_tx: ae.con_mgr_bdlc.tx.clone(),
                            their_bdlc_with_us: bdlcs.their_bdlc_with_us.clone(),
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

        (join_handle, their_bdlc_with_us)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use client::Client;
    use cmd_done::CmdDone;
    use con_mgr::ConMgr;
    use crossbeam_channel::unbounded;
    use echo_requestee_protocol::{EchoReq, EchoRsp};
    //use echo_start_complete_protocol::{EchoComplete, EchoStart};
    use msg_header::BoxMsgAny;
    use server::Server;
    use an_id::AnId;

    use super::*;

    #[test]
    fn test_con_mgr_server() {
        println!("\ntest_con_mgr_server:+");
        let supervisor_instance_id = AnId::new();
        let (supervisor_tx, supervisor_rx) = unbounded::<BoxMsgAny>();

        let con_mgr_name = "con_mgr";
        let con_mgr = Box::new(ConMgr::new(con_mgr_name, &supervisor_instance_id, &supervisor_tx));

        // Start an ActorExecutor
        let (aex1_join_handle, aex1_bdlc) =
            ActorExecutor::start("aex1", con_mgr.their_bdlc_with_us());
        println!("test_con_mgr_server: aex1_bdlc={aex1_bdlc:?}");

        // Add con_mgr to ActorExecutor
        let msg = Box::new(ReqAddActor::new(con_mgr, &supervisor_instance_id));
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_server: sent {} to aex1", con_mgr_name);
        let msg_any = supervisor_rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_server: recvd rsp_add_actor={msg:?}");

        // Add Server s1 to ActorExecutor
        let s1_name = "server1";
        let s1 = Box::new(Server::new(s1_name));
        println!("test_con_mgr_server: create s1={s1:?}");
        let msg = Box::new(ReqAddActor::new(s1, &supervisor_instance_id));
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_server: sent {} to aex1", s1_name);
        let msg_any = supervisor_rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_server: recvd rsp_add_actor={msg:?}");

        let s1_bdlc = &msg.bdlc;

        // Waiting for Server to start, shouldn't have to do this :)
        //println!("Waiting for Server to start, shouldn't have to do this :)");
        //thread::sleep(Duration::from_millis(1000));

        println!("test_con_mgr_server: send EchoReq");
        s1_bdlc.send(Box::new(EchoReq::new(&supervisor_instance_id, 1))).unwrap();
        println!("test_con_mgr_server: sent EchoReq");

        println!("test_con_mgr_server: wait EchoRsp");
        let msg_any = supervisor_rx.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoRsp>().unwrap();
        println!("test_con_mgr_server: recv EchoRsp={msg_rsp:?}");
        assert_eq!(msg_rsp.counter, 1);

        println!("test_con_mgr_server: send CmdDone");
        let msg = Box::new(CmdDone::new());
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_server: sent CmdDone");

        println!("test_con_mgr_server: join aex1 to complete");
        aex1_join_handle.join().unwrap();
        println!("test_con_mgr_server: join aex1 to completed");

        //println!("test_con_mgr_server:-");
    }

    #[test]
    fn test_con_mgr_client_server() {
        println!("\ntest_con_mgr_client_server:+");
        let supervisor_instance_id = AnId::new();
        let (supervisor_tx, supervisor_rx) = unbounded::<BoxMsgAny>();

        let con_mgr_name = "con_mgr";
        let con_mgr = Box::new(ConMgr::new(con_mgr_name, &supervisor_instance_id, &supervisor_tx));

        // Start an ActorExecutor
        let (aex1_join_handle, aex1_bdlc) =
            ActorExecutor::start("aex1", con_mgr.their_bdlc_with_us());
        println!("test_con_mgr_client_server: aex1_bdlc={aex1_bdlc:?}");

        // Add con_mgr to ActorExecutor
        let msg = Box::new(ReqAddActor::new(con_mgr, &supervisor_instance_id));
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent {} to aex1", con_mgr_name);
        let msg_any = supervisor_rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_client_server: recvd rsp_add_actor={msg:?}");

        // Add client1 to ActorExecutor
        let c1_name = "client1";
        let c1 = Box::new(Client::new(c1_name));
        let c1_instance_id = c1.get_instance_id().clone();
        println!("test_con_mgr_client_server: {c1_instance_id} create c1={c1:?}");
        let msg = Box::new(ReqAddActor::new(c1, &supervisor_instance_id));
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent {} to aex1", c1_name);
        let msg_any = supervisor_rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_client_server: recvd rsp_add_actor={msg:?}");

        // Add server1 to ActorExecutor
        let s1_name = "server1";
        let s1 = Box::new(Server::new(s1_name));
        println!("test_con_mgr_client_server: create s1={s1:?}");
        let msg = Box::new(ReqAddActor::new(s1, &supervisor_instance_id));
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent {} to aex1", s1_name);
        let msg_any = supervisor_rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_con_mgr_client_server: recvd rsp_add_actor={msg:?}");

        //// Send EchoStart to c1
        //println!("test_con_mgr_client_server: send EchoStart");
        //c1_bdlc
        //    .send(Box::new(EchoStart::new(s1_bdlc.clone_tx(), 10)))
        //    .unwrap();
        //println!("test_con_mgr_client_server: sent EchoStart");

        //// Wait for EchoComplete from c1
        //println!("test_con_mgr_client_server: wait EchoComplete");
        //let msg_any = c1_bdlc.recv().unwrap();
        //let msg_rsp = msg_any.downcast_ref::<EchoComplete>().unwrap();
        //println!("test_con_mgr_client_server: recv EchoComplete={msg_rsp:?}");

        println!("test_con_mgr_client_server: waiting 1 second");
        thread::sleep(Duration::from_secs(1));

        println!("test_con_mgr_client_server: send CmdDone");
        let msg = Box::new(CmdDone::new());
        aex1_bdlc.send(msg).unwrap();
        println!("test_con_mgr_client_server: sent CmdDone");

        println!("test_con_mgr_client_server: join aex1 to complete");
        aex1_join_handle.join().unwrap();
        println!("test_con_mgr_client_server: join aex1 to completed");

        //drop(supervisor_tx);
        //drop(supervisor_rx);

        println!("test_con_mgr_client_server:-");
    }
}
