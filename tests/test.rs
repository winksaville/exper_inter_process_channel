use std::any::{Any, TypeId};

use msg1::Msg1;
use msg2::{Msg2, MSG2_ID};

#[test]
/// CAREFUL: Deserializing a Msg2 to a Msg1 "works",
/// but it shouldn't as they have different MsgIds.
fn test_identical_json() {
    // Create a Box<Msg2>
    let msg2 = Box::<Msg2>::default();
    let ser_msg2 = serde_json::to_string(&msg2).unwrap();

    // Deserialize to Msg1, this should fail but currently it succeeds
    let deser_bad_msg2 = serde_json::from_str::<Msg1>(&ser_msg2);
    match deser_bad_msg2 {
        Ok(bad_msg2) => {
            println!("test test_identical_json: `serde_json::from_str::<Msg1>(&ser_msg2)` should fail as id is MSG2_ID not MSG1_ID");

            // Rust thinks this is a Msg1
            assert_eq!(TypeId::of::<Msg1>(), bad_msg2.type_id());
            // But the header.id is MSG2_ID
            assert_eq!(bad_msg2.header.id, MSG2_ID);
        }
        Err(_) => panic!("This is expected"),
    }
}
