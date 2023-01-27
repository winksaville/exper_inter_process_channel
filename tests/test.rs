use msg1::Msg1;
use msg2::Msg2;

#[test]
#[should_panic]
/// CAREFUL: Deserializing a Msg2 to a Msg1 "works",
/// but it shouldn't as they have different MsgIds.
fn test_identical_json() {
    // Create a Box<Msg2>
    let msg2 = Box::<Msg2>::default();
    let ser_msg2 = msg2.to_serde_json_string().unwrap();

    // Deserialize to Msg1, this should fail
    let deser_bad_msg2 = Msg1::from_serde_json_str(&ser_msg2);
    match deser_bad_msg2 {
        Some(_) => {
            println!("test test_identical_json: Unexpected success, `Msg1::from_serde_json_str(&ser_msg2)` should fail as id is MSG2_ID not MSG1_ID");
        }
        None => panic!("This is expected"),
    }
}
