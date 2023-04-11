use an_id::AnId;
use box_msg_any::BoxMsgAny;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::cell::UnsafeCell;

#[derive(Clone, Debug)]
pub struct ActorSender {
    pub name: String,
    pub dst_instance_id: AnId,
    pub dst_sndr: Sender<BoxMsgAny>,
}

impl ActorSender {
    pub fn new(name: &str, instance_id: &AnId, tx: Sender<BoxMsgAny>) -> Self {
        Self {
            name: name.to_string() + "_chnl_tx",
            dst_instance_id: *instance_id,
            dst_sndr: tx,
        }
    }

    pub fn send(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.dst_sndr.send(msg)?;
        Ok(())
    }

    pub fn get_dst_instance_id(&self) -> &AnId {
        &self.dst_instance_id
    }
}

#[derive(Clone, Debug)]
pub struct ActorReceiver {
    pub name: String,
    pub rx: Receiver<BoxMsgAny>,
}

impl ActorReceiver {
    pub fn new(name: &str, rx: Receiver<BoxMsgAny>) -> Self {
        Self {
            name: name.to_string() + "_chnl_rx",
            rx,
        }
    }

    pub fn recv(&self) -> Result<BoxMsgAny, Box<dyn std::error::Error>> {
        let msg_any = self.rx.recv()?;
        Ok(msg_any)
    }
}

#[derive(Debug, Clone)]
pub struct ActorChannel {
    pub sender: ActorSender,
    pub receiver: ActorReceiver,
}

impl ActorChannel {
    pub fn new(name: &str, instance_id: &AnId) -> Self {
        let (tx, rx) = unbounded();
        Self {
            sender: ActorSender::new(name, instance_id, tx),
            receiver: ActorReceiver::new(name, rx),
        }
    }
}

#[derive(Debug)]
pub struct VecActorChannel(UnsafeCell<Vec<ActorChannel>>);

impl Default for VecActorChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl VecActorChannel {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    // Panic's if idx is out of bounds
    pub fn get(&self, idx: usize) -> &ActorChannel {
        unsafe {
            let v = &*self.0.get();
            &v[idx]
        }
    }

    pub fn push(&self, chnl: ActorChannel) {
        unsafe {
            let ptr = &mut *self.0.get();
            ptr.push(chnl);
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let v = &*self.0.get();
            v.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use an_id::AnId;
    use msg1::{Msg1, MSG1_ID};
    use msg_header::MsgHeader;

    #[test]
    fn test_actor_channel() {
        let dst_id = AnId::new();
        let supervisor_instance_id = AnId::new();

        let supervisor_chnl = ActorChannel::new("supervisor", &supervisor_instance_id);
        println!("supervisor_chnl: {supervisor_chnl:?}");

        let msg_1 = Box::new(Msg1::new(&dst_id, &supervisor_instance_id, 1));

        assert_eq!("supervisor_chnl_tx", supervisor_chnl.sender.name);
        assert_eq!("supervisor_chnl_rx", supervisor_chnl.receiver.name);

        let print_string = format!("ActorChannel {{ sender: ActorSender {{ name: \"supervisor_chnl_tx\", dst_instance_id: {:?}, dst_sndr: Sender {{ .. }} }}, receiver: ActorReceiver {{ name: \"supervisor_chnl_rx\", rx: Receiver {{ .. }} }} }}", supervisor_instance_id);
        let supervisor_chnl_string = format!("{supervisor_chnl:?}");
        assert_eq!(print_string, supervisor_chnl_string);

        // Verify we can send and receive on supervisor_chnl
        supervisor_chnl.sender.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = supervisor_chnl.receiver.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            msg_1.msg_id()
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.msg_id(), &MSG1_ID);
        assert_eq!(recv_msg_1.dst_id(), &dst_id);
        assert_eq!(recv_msg_1.src_id(), &supervisor_instance_id);
        assert_eq!(recv_msg_1.v, msg_1.v);
    }
}
