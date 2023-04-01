//! This module provides a thread safe hashmap that maps instance_id to a sender.
use std::sync::RwLock;

use an_id::AnId;
use crossbeam_channel::Sender;
use msg_header::BoxMsgAny;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static SENDER_HASHMAP: Lazy<RwLock<HashMap<AnId, Sender<BoxMsgAny>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// Add the sender to the response channel map.
//
// This is thread safe and but only one sender is added per instance_id
// additional invocations will be ignored.
pub fn sender_map_insert(instance_id: &AnId, sender: &Sender<BoxMsgAny>) {
    let mut wlocked_hashmap = SENDER_HASHMAP.write().unwrap(); // TODO: remove unwrap
    if !wlocked_hashmap.contains_key(instance_id) {
        println!("sender_map_insert: instance_id: {}", instance_id);
        let r = wlocked_hashmap.insert(*instance_id, sender.clone());
        assert!(r.is_none());
    }
}

// Get the sender from the response channel map.
pub fn sender_map_get(instance_id: &AnId) -> Option<Sender<BoxMsgAny>> {
    let rlocked_hashmap = SENDER_HASHMAP.read().unwrap(); // TODO: remove unwrap
    let sender = rlocked_hashmap.get(instance_id).cloned();

    println!(
        "sender_map_get: instance_id: {} sender: {:?}",
        instance_id, sender
    );
    sender
}

#[cfg(test)]
mod test {
    use super::*;

    // Test that the sender_map_insert and sender_map_get functions work.
    #[test]
    fn test_sender_map() {
        // Create an instance_id, sender and receiver and .
        let instance_id = AnId::new();
        let (sender, receiver) = crossbeam_channel::unbounded();
        sender_map_insert(&instance_id, &sender);

        // Verify that sender receiver work
        sender.send(Box::new(1)).unwrap();
        let r = receiver.recv().unwrap();
        assert_eq!(r.downcast_ref::<i32>().unwrap(), &1);

        // Verify that sender_map_get works
        let sender2 = sender_map_get(&instance_id).unwrap();
        sender2.send(Box::new(2)).unwrap();
        let r = receiver.recv().unwrap();
        assert_eq!(r.downcast_ref::<i32>().unwrap(), &2);
    }
}
