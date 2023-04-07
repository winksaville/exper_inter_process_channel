//! This module provides a thread safe hashmap that maps instance_id to a sender.
use std::sync::RwLock;

use actor_channel::ActorSender;
use an_id::AnId;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static SENDER_HASHMAP: Lazy<RwLock<HashMap<AnId, ActorSender>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// Add the sender to the response channel map.
//
// This is thread safe and but only one sender is added per instance_id
// additional invocations will be ignored.
pub fn sender_map_insert(instance_id: &AnId, sender: &ActorSender) {
    let mut wlocked_hashmap = SENDER_HASHMAP.write().unwrap(); // TODO: remove unwrap
    if !wlocked_hashmap.contains_key(instance_id) {
        println!("sender_map_insert: instance_id: {}", instance_id);

        // Actually, this doesn't solve the problem, is seems
        // to be associated with a "false" circular dependency.

        //// These needed to satisfy the rust analyzer type checker.
        //let v: ActorSender = sender.clone();
        //let r: Option<ActorSender> = wlocked_hashmap.insert(*instance_id, v);

        // This is also "correct code" and compiles and runs fine,
        // but as of v1.62 of rustc rust-analyzer generates a type-mismatch.
        // on uses of this function
        let r: Option<ActorSender> = wlocked_hashmap.insert(*instance_id, sender.clone());

        assert!(r.is_none());
    } else {
        panic!("sender_map_insert: instance_id: {} already exists", instance_id);
    }
}

// Get the sender from the response channel map.
pub fn sender_map_get(instance_id: &AnId) -> Option<ActorSender> {
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
    use actor_channel::ActorChannel;

    use super::*;

    // Test that the sender_map_insert and sender_map_get functions work.
    #[test]
    fn test_sender_map() {
        // Create an instance_id, sender and receiver and .
        let instance_id = AnId::new();
        let ac = ActorChannel::new("test_sender_map");
        sender_map_insert(&instance_id, &ac.sender);

        // Verify that sender receiver work
        ac.sender.send(Box::new(1)).unwrap();
        let r = ac.receiver.recv().unwrap();
        assert_eq!(r.downcast_ref::<i32>().unwrap(), &1);

        // Verify that sender_map_get works
        let sender2 = sender_map_get(&instance_id).unwrap();
        sender2.send(Box::new(2)).unwrap();
        let r = ac.receiver.recv().unwrap();
        assert_eq!(r.downcast_ref::<i32>().unwrap(), &2);
    }
}
