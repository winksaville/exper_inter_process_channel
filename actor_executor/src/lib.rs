use std::thread::{self, JoinHandle};

use actor::Actor;
use actor_bi_dir_channel::{
    vec_bi_dir_local_channels::{BiDirLocalChannels, VecBdlcs},
    ActorBiDirChannel, BiDirLocalChannel,
};
use cmd_done::CmdDone;
use crossbeam_channel::Select;
//use msg_header::BoxMsgAny;
use req_add_actor::ReqAddActor;
use req_their_bi_dir_channel::ReqTheirBiDirChannel;
use rsp_add_actor::RspAddActor;
use rsp_their_bi_dir_channel::RspTheirBiDirChannel;

#[derive(Debug)]
struct ActorsExecutor {
    pub name: String,
    pub actor_vec: Vec<Box<dyn Actor>>,
    pub bi_dir_channels_vec: VecBdlcs, //Vec<Box<BiDirLocalChannels>>,
    done: bool,
}

#[allow(unused)]
impl ActorsExecutor {
    // Returns a thread::JoinHandle and a Box<dyn ActorBiDirChannel> which
    // allows messages to be sent and received from the AeActor.
    pub fn start(name: &str) -> (JoinHandle<()>, Box<BiDirLocalChannel>) {
        let ae_actor_bi_dir_channels = BiDirLocalChannels::new();
        let their_bi_dir_channel = Box::new(ae_actor_bi_dir_channels.their_channel.clone());

        // Convert name to string so it can be moved into the thread
        let name = name.to_string();

        let join_handle = thread::spawn(move || {
            let mut ae = Self {
                name: name.to_string(),
                actor_vec: Vec::new(),
                bi_dir_channels_vec: VecBdlcs::new(),
                done: false,
            };
            println!("AE:{}:+", ae.name);

            let mut selector = Select::new();
            let oper_idx = selector.recv(ae_actor_bi_dir_channels.our_channel.get_recv());
            assert_eq!(oper_idx, 0);

            while !ae.done {
                println!("AE:{}: TOL", ae.name);
                let oper = selector.select();
                let oper_idx = oper.index();

                if oper_idx == 0 {
                    // This messageis for the AE itself
                    let rx = ae_actor_bi_dir_channels.our_channel.get_recv();

                    let result = oper.recv(rx);
                    match result {
                        Err(why) => {
                            // TODO: Error on our selves make done, is there anything else we need to do?
                            println!("AE:{}: error on recv: {why} `done = true`", ae.name);
                            ae.done = true;
                        }
                        Ok(msg_any) => {
                            // This is a message for this ActorExecutor!!!
                            println!("{}: msg_any={msg_any:?}", ae.name);
                            if msg_any.downcast_ref::<ReqAddActor>().is_some() {
                                // It is a MsgReqAeAddActor, now downcast to concrete message so we can push it to actor_vec
                                let msg = msg_any.downcast::<ReqAddActor>().unwrap();
                                println!("{}: msg={msg:?}", ae.name);
                                let actor_idx = ae.actor_vec.len();
                                ae.actor_vec.push(msg.actor);

                                // Create the bdlcs and add to bi_dir_channels_vec
                                println!("{}: create BiDirLocalChannels", ae.name());
                                let bdlcs = BiDirLocalChannels::new();

                                assert_eq!(ae.bi_dir_channels_vec.len(), actor_idx);

                                ae.bi_dir_channels_vec.push(bdlcs);
                                let bdlcs = ae.bi_dir_channels_vec.get(actor_idx);

                                println!("{}: selector.recv(our_channel.get_recv())", ae.name());
                                selector.recv(bdlcs.our_channel.get_recv());

                                // Send the response message with their_channel
                                let msg_rsp = Box::new(RspAddActor::new(Box::new(
                                    bdlcs.their_channel.clone(),
                                )));
                                println!("{}: msg.rsp_tx.send msg={msg_rsp:?}", ae.name());
                                msg.rsp_tx.send(msg_rsp);

                                println!(
                                    "{}: added new receiver for {}",
                                    ae.name(),
                                    ae.actor_vec[actor_idx].get_name()
                                );
                            } else if let Some(msg) = msg_any.downcast_ref::<CmdDone>() {
                                println!("{}: msg={msg:?}", ae.name());
                                ae.done = true;
                            } else if let Some(msg) = msg_any.downcast_ref::<ReqTheirBiDirChannel>()
                            {
                                println!("{}: msg={msg:?}", ae.name());
                                let bdc = ae.bi_dir_channels_vec.get(msg.handle);
                                let their_channel = bdc.their_channel.clone();
                                let msg_rsp =
                                    Box::new(RspTheirBiDirChannel::new(Box::new(their_channel)));

                                // send msg_rsp
                                println!("{}: send msg_rsp={msg_rsp:?}", ae.name());
                                msg.rsp_tx.send(msg_rsp).unwrap();
                            } else {
                                println!("{}: Uknown msg", ae.name());
                            }
                        }
                    }
                } else {
                    // This message for one of the actors running in the AE
                    let actor_idx = oper_idx - 1;
                    let actor = &mut ae.actor_vec[actor_idx];
                    println!(
                        "{}: msg for actor_vec[{actor_idx}] {}",
                        ae.name,
                        actor.get_name()
                    );
                    let bdlcs = ae.bi_dir_channels_vec.get(actor_idx);
                    let rx = bdlcs.our_channel.get_recv();
                    if let Ok(msg_any) = oper.recv(rx).map_err(|why| {
                        // TODO: What should we do here?
                        panic!("{}: {} error on recv: {why}", ae.name, actor.get_name())
                    }) {
                        actor.process_msg_any(Some(&bdlcs.our_channel.tx), msg_any);
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

        (join_handle, their_bi_dir_channel)
    }

    fn name(&self) -> &str {
        // This needs an InstanceId
        "ActorExecutor"
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;
    use cmd_done::CmdDone;
    use echo_requestee_protocol::{EchoReq, EchoReply};
    use msg_header::BoxMsgAny;
    use server::Server;

    use super::*;

    #[test]
    fn test_msg_req_add_actor() {
        println!("\ntest_msg_req_add_actor:+");
        let (tx, rx) = unbounded::<BoxMsgAny>();

        // Start an ActorsExecutor
        let (executor1_join_handle, executor1_tx) = ActorsExecutor::start("executor1");
        println!("test_msg_req_add_actor: executor1_tx={executor1_tx:?}");

        // Create Actor Server
        let s1 = Box::new(Server::new("s1"));
        println!("test_msg_req_add_actor: create s1={s1:?}");

        let s1_name = s1.get_name().to_owned();

        // Add Thing to the executor
        let msg = Box::new(ReqAddActor::new(s1, tx));
        executor1_tx.send(msg).unwrap();
        println!("test_msg_req_add_actor: sent {} to executor1", s1_name);

        let msg_any = rx.recv().unwrap();
        let msg = msg_any.downcast_ref::<RspAddActor>().unwrap();
        println!("test_msg_req_add_actor: recvd rsp_add_actor={msg:?}");

        let s1_bdlc = &msg.bdlc;

        println!("test_msg_req_add_actor: send EchoReq");
        s1_bdlc.send(Box::new(EchoReq::new(1))).unwrap();
        println!("test_msg_req_add_actor: sent EchoReq");

        println!("test_msg_req_add_actor: wait EchoRsp");
        let msg_any = s1_bdlc.recv().unwrap();
        let msg_rsp = msg_any.downcast_ref::<EchoReply>().unwrap();
        println!("test_msg_req_add_actor: recv EchoReply={msg_rsp:?}");
        assert_eq!(msg_rsp.counter, 1);

        println!("test_msg_req_add_actor: send CmdDone");
        let msg = Box::new(CmdDone::new());
        executor1_tx.send(msg).unwrap();
        println!("test_msg_req_add_actor: sent CmdDone");

        println!("test_msg_req_add_actor: join executor1 to complete");
        executor1_join_handle.join().unwrap();
        println!("test_msg_req_add_actor: join executor1 to completed");

        println!("test_msg_req_add_actor:-");
    }
}
