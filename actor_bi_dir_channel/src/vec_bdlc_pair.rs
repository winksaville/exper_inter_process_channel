//! This file defines `struct BiDirLocalChannels` which is used
//! for bi-directional communication between actors. In particular
//! by defining a bi-directional channel an entity that invokes an
//! actors `process_msg_any` has available the `rsp_tx` and it reduces
//! the need to pass a `Channel` in messages. Most importantly when
//! an inter process communication channel is created directly passing
//! of a channel would not be possible, this will eliminate that need.
//!
//! Note: this is a separate file because it uses UnsafeCell.
use std::cell::UnsafeCell;

use crossbeam_channel::unbounded;

use crate::BiDirLocalChannel;

#[derive(Debug, Clone)]
pub struct BdlcPair {
    pub their_channel: BiDirLocalChannel,
    pub our_channel: BiDirLocalChannel,
}

impl Default for BdlcPair {
    fn default() -> Self {
        Self::new()
    }
}

impl BdlcPair {
    pub fn new() -> Self {
        // left_tx -----> right_rx
        let (left_tx, right_rx) = unbounded();

        // left_rx <---- right_tx
        let (right_tx, left_rx) = unbounded();

        Self {
            their_channel: BiDirLocalChannel {
                self_tx: right_tx.clone(),
                tx: left_tx.clone(),
                rx: left_rx,
            },
            our_channel: BiDirLocalChannel {
                self_tx: left_tx,
                tx: right_tx,
                rx: right_rx,
            },
        }
    }
}

#[derive(Debug)]
pub struct VecBdlcPair(UnsafeCell<Vec<BdlcPair>>);

impl Default for VecBdlcPair {
    fn default() -> Self {
        Self::new()
    }
}

impl VecBdlcPair {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    // Panic's if idx is out of bounds
    pub fn get(&self, idx: usize) -> &BdlcPair {
        unsafe {
            let v = &*self.0.get();
            &v[idx]
        }
    }

    pub fn push(&self, bdlcs: BdlcPair) {
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
