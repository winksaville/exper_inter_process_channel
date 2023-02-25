

#[cfg(feature = "simple")]
fn simple() {
    let mut channels: Vec::<(Sender<i32>, Receiver<i32>)> = Vec::new();
    let mut sel = Select::new();

    for i in 0..=1 {
        let (tx, rx) = unbounded::<i32>();
        channels.push((tx, rx));
        sel.recv(&channels[i].1);
    }
}

#[cfg(feature = "rc_refcell")]
fn rc_refcell() {
    use std::{rc::Rc, cell::RefCell};

    struct Channel {
        tx: Sender<i32>,
        rx: Receiver<i32>,
    }

    let mut channels: Vec::<Rc<RefCell<Channel>>> = Vec::new();
    let mut sel = Select::new();

    for i in 0..=1 {
        let (tx, rx) = unbounded::<i32>();
        channels.push(Rc::new(RefCell::new(Channel {tx, rx})));
        let channel = &(*channels)[i].borrow();
        sel.recv(&channel.rx);
    }
}

use std::cell::UnsafeCell;
use crossbeam_channel::{Sender, Receiver, Select, unbounded};

#[allow(unused)]
#[derive(Debug)]
struct Channel {
    tx: Sender<i32>,
    rx: Receiver<i32>,
}

#[allow(unused)]
#[derive(Debug)]
struct VecChannel(UnsafeCell<Vec<Channel>>);

impl VecChannel {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    // Panic's if idx is out of bounds
    pub fn get(&self, idx: usize) -> &Channel {
        let channel = unsafe {
            let v = &*self.0.get();
            &v[idx]
        };
        channel
    }

    pub fn push(&self, channel: Channel) {
        unsafe {
            let v = &mut *self.0.get();
            v.push(channel);
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe {
            let v = &*self.0.get();
            v.len()
        };

        len
    }
}

fn working() {
    let channels = VecChannel::new();
    let mut sel = Select::new();

    // Incrementally add a receiver and test it
    for i in 0..=1 {
        let (tx, rx) = unbounded::<i32>();
        channels.push(Channel {tx, rx});
        let channel = channels.get(i);
        sel.recv(&channel.rx);

        // Test

        // Send a value via tx
        let send_value = 10 + (i as i32);
        println!("send: {send_value}");
        channel.tx.send(send_value).unwrap();

        // "wait" for something to arrive on a channel and
        // get operator index, validated it's the expected index
        let oper = sel.select();
        println!("oper.index(): {}", oper.index());
        let expected_oper_index = channels.len() - 1;
        assert_eq!(oper.index(), expected_oper_index);

        // Now receove the data and validate it's the sent value
        let value = oper.recv(&channel.rx).unwrap();
        println!("recv: {value}");
        assert_eq!(value, send_value);
    }
}

fn main() {
    #[cfg(feature = "simple")]
    simple();

    #[cfg(feature = "rc_refcell")]
    rc_refcell();

    working();
}