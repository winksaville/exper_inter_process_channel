use an_id::AnId;
use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct NameId {
    id: AnId,
    name: String,
}

impl Debug for NameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("NameId")
                .field("name", &self.name)
                .field("id", &self.id)
                .finish()
        } else {
            write!(f, "{}", self.name())
        }
    }
}

impl Display for NameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl NameId {
    pub fn new(name: &str, id: AnId) -> Self {
        Self {
            name: name.to_string(),
            id,
        }
    }

    pub fn new_v4(name: &str) -> Self {
        NameId::new(name, AnId::new())
    }

    pub fn id(&self) -> &AnId {
        &self.id
    }

    pub fn name(&self) -> String {
        format!("{}-{}", self.name, &self.id.to_string()[0..8])
    }

    pub fn just_name(&self) -> &str {
        &self.name
    }

    pub fn full_name(&self) -> String {
        format!("{}-{}", self.name, self.id)
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn test_name_id() {
        println!("test_name_id");
        let id = AnId::new();
        let nid = NameId::new("test", id.clone());
        println!("test_name_id: {nid}");
        println!("test_name_id: {nid:?}");
        println!("test_name_id: {nid:#?}");
        assert_eq!(nid.just_name(), "test");
        assert_eq!(nid.id(), &id);
        assert_eq!(
            nid.name(),
            format!("{}-{}", &nid.name, &id.to_string()[0..8])
        );
        assert_eq!(
            nid.full_name(),
            format!("{}-{}", &nid.name, &id.to_string())
        );
    }

    #[test]
    fn test_struct_with_name_id() {
        #[allow(unused)]
        #[derive(Clone, Debug)]
        struct NoNameId {
            f: f32,
            i: u32,
        }

        #[allow(unused)]
        #[derive(Clone, Debug)]
        struct Other {
            no_name_id: NoNameId,
            nested_id: NameId,
            i: u32,
        }

        #[allow(unused)]
        #[derive(Clone, Debug)]
        struct TestStruct {
            id: NameId,
            s: String,
            other: Other,
        }

        println!("test_struct_name_id");
        let no_name_id = NoNameId { f: 123.0, i: 123 };
        let other = Other {
            no_name_id,
            nested_id: NameId::new_v4("other"),
            i: 123,
        };
        let test_struct = TestStruct {
            id: NameId::new_v4("test_struct"),
            s: "hello".to_string(),
            other: other.clone(),
        };
        println!("other: {other:?}");
        println!("other: {other:#?}");
        println!("test_struct: {test_struct:?}");
        println!("test_struct: {test_struct:#?}");
    }
}
