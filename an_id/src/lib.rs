//! For the framework this is "guranteed" to be unique
//! across all systems using it. Currently it's a Uuid
//! but that could change hence this New Type.
pub use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
pub use uuid::uuid;
use uuid::Uuid;

#[macro_export]
macro_rules! anid {
    ($id_str:literal) => {
        paste! {
            #[allow(unused)]
            an_id::AnId(uuid::uuid!($id_str))
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnId(pub Uuid);

impl Debug for AnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "InstanceId({})", self.0)
        } else {
            write!(f, "{}", &self.0.to_string()[0..8])
        }
    }
}

impl Display for AnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Handle precision
        let s = if let Some(precision) = f.precision() {
            self.0.to_string()[0..precision].to_string()
        } else {
            self.0.to_string()
        };

        // Handle alignment padding
        f.pad(&s)
    }
}

impl Default for AnId {
    fn default() -> Self {
        Self::new()
    }
}

impl AnId {
    pub fn new() -> Self {
        AnId::from(Uuid::new_v4())
    }

    pub fn from(id: Uuid) -> Self {
        Self(id)
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_an_id() {
        println!("test_an_id");
        let id = AnId::new();
        let from_id = AnId::from(id.0);
        println!("test_an_id:        {{id}}={id}");
        println!("test_an_id:    {{id:.36}}={id:.36}");
        println!("test_an_id:  {{id:36.34}}={id:36.34}");
        println!("test_an_id:  {{id:<36.8}}={id:<36.8}");
        println!("test_an_id:  {{id:>36.8}}={id:>36.8}");
        println!("test_an_id:  {{id:^36.8}}={id:^36.8}");
        println!("test_an_id:     {{id:.8}}={id:.8}");
        println!("test_an_id:      {{id:?}}={id:?}");
        println!("test_an_id:     {{id:#?}}={id:#?}");
        assert_eq!(id, from_id);
    }
}
