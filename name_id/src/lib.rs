use std::fmt::{Debug, Display};
use uuid::Uuid;

#[derive(Clone)]
pub struct NameId {
    name: String,
    short_name: String,
    full_name: String,
    id: Uuid,
}

impl Debug for NameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("NameId")
                .field("name", &self.name)
                .field("short_name", &self.short_name)
                .field("full_name", &self.full_name)
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
    pub fn new(name: &str, id: Uuid) -> Self {
        Self {
            name: name.to_string(),
            short_name: format!("{}-{}", name, &id.to_string()[0..8]),
            full_name: format!("{}-{}", name, &id.to_string()),
            id,
        }
    }

    pub fn new_v4(name: &str) -> Self {
        NameId::new(name, Uuid::new_v4())
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[allow(clippy::misnamed_getters)]
    pub fn name(&self) -> &str {
        &self.short_name
    }

    pub fn just_name(&self) -> &str {
        &self.name
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn test_name_id() {
        println!("test_name_id");
        let id = Uuid::new_v4();
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
