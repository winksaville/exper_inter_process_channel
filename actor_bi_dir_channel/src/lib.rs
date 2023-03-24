use std::cell::UnsafeCell;

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
    pub self_tx: Sender<BoxMsgAny>,
    pub tx: Sender<BoxMsgAny>,
    pub rx: Receiver<BoxMsgAny>,
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

#[derive(Debug, Clone)]
pub struct Connection {
    pub their_bdlc_with_us: BiDirLocalChannel,
    pub our_bdlc_with_them: BiDirLocalChannel,
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

impl Connection {
    pub fn new() -> Self {
        let (their_bdlc_with_us, our_bdlc_with_them) = BiDirLocalChannel::new();
        Self {
            their_bdlc_with_us,
            our_bdlc_with_them,
        }
    }
}

#[derive(Debug)]
pub struct VecConnection(UnsafeCell<Vec<Connection>>);

impl Default for VecConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl VecConnection {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    // Panic's if idx is out of bounds
    pub fn get(&self, idx: usize) -> &Connection {
        unsafe {
            let v = &*self.0.get();
            &v[idx]
        }
    }

    pub fn push(&self, bdlcs: Connection) {
        unsafe {
            let ptr = &mut *self.0.get();
            ptr.push(bdlcs);
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
    use msg1::Msg1;
    use msg_header::MsgHeader;

    use super::*;

    #[test]
    fn test_bi_dir_local_channel() {
        let (left, right) = BiDirLocalChannel::new();

        let msg_1 = Box::new(Msg1::new(1));

        // Verify left can send to self and receive on left.recv
        left.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = left.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify right can send to self and receive on right.recv
        right.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = right.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify left can send to right
        left.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = right.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify right can send to left
        right.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = left.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify both left and right recv are empty
        assert!(left.rx.try_recv().is_err());
        assert!(right.rx.try_recv().is_err());
    }

    #[test]
    fn test_connection() {
        let connection = Connection::new();

        let msg_1 = Box::new(Msg1::new(1));

        // Verify their_bdlc_with_us can send to self
        connection.their_bdlc_with_us.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = connection.their_bdlc_with_us.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify our_bdlc_with_them can send to self and receive on our_bdlc_with_them.recv
        connection.our_bdlc_with_them.send_self(msg_1.clone()).unwrap();
        let recv_msg_1_any = connection.our_bdlc_with_them.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify their_bdlc_with_us can send to our_bdlc_with_them
        connection.their_bdlc_with_us.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = connection.our_bdlc_with_them.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify our_bdlc_with_them can send to their_bdlc_with_us
        connection.our_bdlc_with_them.send(msg_1.clone()).unwrap();
        let recv_msg_1_any = connection.their_bdlc_with_us.recv().unwrap();
        assert_eq!(
            MsgHeader::get_msg_id_from_boxed_msg_any(&recv_msg_1_any),
            &msg_1.header.msg_id
        );
        let recv_msg_1 = recv_msg_1_any.downcast_ref::<Msg1>().unwrap();
        assert_eq!(recv_msg_1.v, msg_1.v);

        // Verify both their_bdlc_with_us and our_bdlc_with_them recv are empty
        assert!(connection.their_bdlc_with_us.rx.try_recv().is_err());
        assert!(connection.our_bdlc_with_them.rx.try_recv().is_err());
    }
}
