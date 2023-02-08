//! Manager
//!
//! This is a quick and dirty Actor Manager, one concern is
//! how big an ID should be. Currently I'm going to use Uuid 128bit
//! (16 bytes) it's certainlly big enough and actually a u64
//! ID is big enough for experiments, but I don't think big enough
//! for the real world, so going with Uuid. Long term it might be
//! better to use IPFS CIDs which are 256bit (32 bytes).
use std::{collections::HashMap, error::Error, fmt::Debug};

use actor::{Actor, ActorId, ActorInstanceId};
use protocol::ProtocolId;
use protocol_set::ProtocolSetId;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ManagerId(pub Uuid);

// Manager
pub struct Manager {
    name: String,
    id: ManagerId,
    actors: Vec<Box<dyn Actor + 'static>>,
    actors_map_by_instance_id: HashMap<ActorInstanceId, usize>,
    actors_map_by_actor_id: HashMap<ActorId, Vec<usize>>,
    actors_map_by_protocol_id: HashMap<ProtocolId, Vec<usize>>,
    actors_map_by_protocol_set_id: HashMap<ProtocolSetId, Vec<usize>>,
}

impl Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = "[".to_owned();
        for (i, a) in self.actors.iter().enumerate() {
            if i != 0 {
                s += " "
            }

            s += &a.get_name_and_short_instance_id();
            s += ",";
        }
        s += "]";

        f.debug_struct("Manager")
            .field("name", &self.name)
            .field("id", &self.id)
            .field("actors", &s)
            .field("actors_map_by_instance_id", &self.actors_map_by_instance_id)
            .field("actors_map_by_actor_id", &self.actors_map_by_actor_id)
            .field("actors_map_by_protocol_id", &self.actors_map_by_protocol_id)
            .field(
                "actors_map_by_protocol_set_id",
                &self.actors_map_by_protocol_set_id,
            )
            .finish()
    }
}

// Allow `clippy::uninlined_format_args`  because in msg_macro
// we need to use stringify!($name) which can't be used in a
// format string. Also this is caught by `cargo +nightly clippy`.
#[allow(clippy::uninlined_format_args)]
impl Manager {
    pub fn new(name: &str, id: ManagerId) -> Self {
        Self {
            name: name.to_string(),
            id,
            actors: Vec::new(),
            actors_map_by_instance_id: HashMap::new(),
            actors_map_by_actor_id: HashMap::new(),
            actors_map_by_protocol_id: HashMap::new(),
            actors_map_by_protocol_set_id: HashMap::new(),
        }
    }

    /// Add an Actor.
    pub fn add_actor(&mut self, actor: impl Actor + 'static) -> Result<(), Box<dyn Error>> {
        #[cfg(debug)]
        println!("Clone only in debug configuration");
        #[cfg(debug)]
        let actor_clone_for_panic = actor.clone();

        let idx = self.actors.len();

        if let Some(idx) = self
            .actors_map_by_instance_id
            .insert(actor.get_instance_id().clone(), idx)
        {
            return Err(format!(
                "{}-{}::add_actor {} instance_id:{} : Actor already added at idx: {}",
                self.name,
                self.id.0,
                actor.get_name(),
                actor.get_instance_id().0,
                idx
            )
            .into());
        }

        let boxed_actor = Box::new(actor);
        self.actors.push(boxed_actor);

        self.add_map_by_actor_id(idx);
        self.add_map_by_protocol_id(idx);
        self.add_map_by_protocol_set_id(idx);

        Ok(())
    }

    fn add_map_by_actor_id(&mut self, idx: usize) {
        let actor = &self.actors[idx];

        if let Some(v) = self.actors_map_by_actor_id.get_mut(actor.get_actor_id()) {
            // Add another idx
            v.push(idx);
        } else {
            // First time seeing this actor_id, add vector with one item
            self.actors_map_by_actor_id
                .insert(actor.get_actor_id().clone(), vec![idx]);
        }
    }

    fn add_map_by_protocol_id(&mut self, idx: usize) {
        let actor = &self.actors[idx];
        let protocol_map = &actor.get_protocol_set().protocols_map;

        for k in protocol_map.keys() {
            if let Some(v) = self.actors_map_by_protocol_id.get_mut(k) {
                v.push(idx);
            } else {
                // First time seeing this protocol_id, add vector with one item
                self.actors_map_by_protocol_id.insert(k.clone(), vec![idx]);
            }
        }
    }

    fn add_map_by_protocol_set_id(&mut self, idx: usize) {
        let actor = &self.actors[idx];

        let protocol_set_id = &actor.get_protocol_set().id;
        if let Some(v) = self.actors_map_by_protocol_set_id.get_mut(protocol_set_id) {
            v.push(idx);
        } else {
            // First time seeing this protocol_set_id, add vector with one item
            self.actors_map_by_protocol_set_id
                .insert(protocol_set_id.clone(), vec![idx]);
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use super::*;
    use client::Client;
    use server::Server;
    use uuid::Uuid;

    #[test]
    fn test_manager() {
        println!("test_manager");
        let mut manager = Manager::new("a_manager", ManagerId(Uuid::new_v4()));

        let (tx, rx) = channel();
        let client = Client::new("client", tx.clone(), tx.clone());
        manager.add_actor(client).unwrap();

        let server1 = Server::new("server1");
        manager.add_actor(server1).unwrap();
        let server2 = Server::new("server2");
        manager.add_actor(server2).unwrap();

        println!("manager: {manager:#?}");

        drop(tx);
        drop(rx);
    }
}
