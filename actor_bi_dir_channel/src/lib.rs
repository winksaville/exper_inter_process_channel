use crossbeam_channel::{unbounded, Receiver, Sender};

use msg_header::BoxMsgAny;

pub trait ActorBiDirChannel {
    fn clone_tx(&self) -> Sender<BoxMsgAny> {
        panic!("ActorBiDirChannel `fn send_self` not implemented");
    }

    fn send_self(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        Err("ActorBiDirChannel `fn send_self` not implemented".into())
    }
    fn send(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        Err("ActorBiDirChannel `fn send` not implemented".into())
    }

    fn recv(&self) -> Result<BoxMsgAny, Box<dyn std::error::Error>> {
        Err("ActorBiDirChannel `fn recv` not implemented".into())
    }

    fn get_recv(&self) -> &Receiver<BoxMsgAny> {
        panic!("ActorBiDirChannel `fn get_recv` not implemented");
    }
}

#[derive(Debug, Clone)]
pub struct BiDirLocalChannel {
    self_tx: Sender<BoxMsgAny>,
    tx: Sender<BoxMsgAny>,
    rx: Receiver<BoxMsgAny>,
}

impl BiDirLocalChannel {
    pub fn new() -> (Self, Self) {
        // left_tx -----> right_rx
        let (left_tx, right_rx) = unbounded();

        // left_rx <---- right_tx
        let (right_tx, left_rx) = unbounded();

        (
            Self {
                self_tx: right_tx.clone(),
                tx: left_tx.clone(),
                rx: left_rx,
            },
            Self {
                self_tx: left_tx,
                tx: right_tx,
                rx: right_rx,
            },
        )
    }
}

impl ActorBiDirChannel for BiDirLocalChannel {
    fn clone_tx(&self) -> Sender<BoxMsgAny> {
        self.tx.clone()
    }

    fn get_recv(&self) -> &Receiver<BoxMsgAny> {
        &self.rx
    }

    fn send_self(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.self_tx
            .send(msg)
            .map_err(|err| format!("Error send_self: {err}").into())
    }

    fn send(&self, msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        self.tx
            .send(msg)
            .map_err(|err| format!("Error send: {err}").into())
    }

    fn recv(&self) -> Result<BoxMsgAny, Box<dyn std::error::Error>> {
        self.rx
            .recv()
            .map_err(|err| format!("Error recv: {err}").into())
    }
}

#[cfg(test)]
mod test {
    use msg1::Msg1;
    use msg_header::MsgHeader;

    use super::*;

    #[test]
    fn test_actor_bi_dir_local_channel() {
        let (left, right) = BiDirLocalChannel::new();

        let msg_1 = Box::new(Msg1::new(1));

        // Verify left can send to self and receive on left.recv
        left.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = left.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify right can send to self and receive on right.recv
        right.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = right.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify left can send to right
        left.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = right.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify right can send to left
        right.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = left.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify both left and right recv are empty
        assert!(left.rx.try_recv().is_err());
        assert!(right.rx.try_recv().is_err());
    }
}
