use std::{thread, collections::HashMap};

use actor::{Actor, ActorId, ActorInstanceId};
use actor_bi_dir_channel::{ActorBiDirChannel, BiDirLocalChannel};
use crossbeam_channel::Select;

use manager::Manager;
use msg_local_macro::{msg_local_macro, paste};

// https://www.uuidgenerator.net/version4
msg_local_macro!(AddActor "828ee6a0-488a-43b6-850a-537820c546ac" {
    actor: Box<ActorInstanceId>
});

impl AddActor {
    pub fn new(actor: Box<ActorInstanceId>) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ADD_ACTOR_ID },
            actor,
        }
    }
}

// https://www.uuidgenerator.net/version4
msg_local_macro!(RemoveActor "1819dc5b-cfdb-439b-82a0-bf48a9f0ecd0" {
    actor_id: ActorId
});

impl RemoveActor {
    pub fn new(actor_id: &ActorId) -> Self {
        let aid = *actor_id;
        Self {
            header: msg_header::MsgHeader { id: REMOVE_ACTOR_ID },
            actor_id: aid,
        }
    }
}

// https://www.uuidgenerator.net/version4
msg_local_macro!(ReplyActorBiDirChannel "75922484-4974-44f9-8eca-1608d965f97e" {
    bi_dir_channel: Box<dyn ActorBiDirChannel>
});

impl ReplyActorBiDirChannel {
    pub fn new(bi_dir_channel: Box<dyn ActorBiDirChannel>) -> Self {
        Self {
            header: msg_header::MsgHeader { id: REMOVE_ACTOR_ID },
            bi_dir_channel,
        }
    }
}

pub struct ActorThread {
    pub ctrl_chnl: Box<dyn ActorBiDirChannel>,
    //pub thread: JoinHandle<()>,
}

impl ActorThread {
    pub fn new(manager: &Manager) -> Self {

        let (ctrl_chnl_left, ctrl_chnl_right) = BiDirLocalChannel::new();

        let this_thread = Self {
            ctrl_chnl: Box::new(ctrl_chnl_left),
        };

        thread::spawn(move || {
            println!("ActorThread:+");
            println!("ActorThread: waiting");

            let mut sel = Select::new();

            //let mut actors = Vec::<Option<(Box<dyn ActorBiDirChannel>, Box<dyn Actor>)>>::new();
            let mut actors_by_id_map = HashMap::<ActorInstanceId, usize>::new();

            let ctrl_chnl_idx = sel.recv(ctrl_chnl_right.get_recv());
            assert_eq!(ctrl_chnl_idx, 0);

            loop {
                let oper = sel.select();

                match oper.index() {
                    0 => {
                        if let Ok(msg_any) = oper.recv(ctrl_chnl_right.get_recv()) {
                            if let Some(msg) = msg_any.downcast_ref::<AddActor>() {
                                assert_eq!(msg.header.id, ADD_ACTOR_ID);

                                let (left, right) = BiDirLocalChannel::new();
                                let oper_idx = sel.recv(right.get_recv());
                                let actors_idx = actors.len();
                                if oper_idx == actors_idx {
                                    panic!("oper_idx: {oper_idx} != actors_idx: {actors_idx} actor name={}, actor instance id={:?}", msg.actor.get_name(), msg.actor.get_instance_id());
                                }
                                actors_by_id_map.insert(*msg.actor.get_instance_id(), actors_idx);

                                actors.push(Some((Box::new(right), msg.actor)));

                                // Send back bi dir channel to actor
                                let reply_msg = Box::new(ReplyActorBiDirChannel::new(Box::new(left)));
                                ctrl_chnl_right.send(reply_msg);

                            } else if let Some(msg) = msg_any.downcast_ref::<RemoveActor>() {
                                assert_eq!(msg.header.id, REMOVE_ACTOR_ID);
                            }
                        }
                    }
                    _ => {
                        let actor_idx = oper.index() - 1;
                        if let Some((actor_recv, actor)) = actors.get_mut(actor_idx) {
                            if let Ok(msg) = oper.recv(actor_recv) {
                                actor.process_msg_any(None, msg);
                            }
                        }
                    }
                }
            }
            //println!("ActorThread:-");

        });

        this_thread
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_actor_thread() {
        println!("test_actor_thread:+");

        //let srvr = Server::new("srvr");
        //let clnt = Client::new("clnt");

        println!("creating thread");
        let actor_thread = ActorThread::new();
        //thread::sleep(Duration::from_millis(100));
        println!("unwaiting thread");
        //actor_thread.waiting_start_tx.send(()).unwrap();
        thread::sleep(Duration::from_millis(100));

        println!("test_actor_thread:-");
    }
}
