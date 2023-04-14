use an_id::AnId;
use msg1::Msg1;
use msg2::Msg2;

#[test]
fn test_to_from_serde_success() {
    let supervisor_instance_id = AnId::new();

    // Create a Box<Msg2>
    let msg2 = Box::new(Msg2::new(&AnId::nil(), &supervisor_instance_id));
    let ser_msg2 = Msg2::to_serde_json_buf(msg2).unwrap();

    // Deserialize to Msg2, this should succeed!
    let deser_m2 = Msg2::from_serde_json_buf(&ser_msg2);
    match deser_m2 {
        Some(m2) => println!("test test_to_from_serde_success: Expected success, m2={m2:?}"),
        None => panic!("from_serded_json_buf(&ser_msg2) failed, this is unexpected"),
    }
}

#[test]
#[should_panic]
fn test_to_from_serde_failure() {
    let supervisor_instance_id = AnId::new();

    // Create a Box<Msg2>
    let msg2 = Box::new(Msg2::new(&AnId::nil(), &supervisor_instance_id));
    let ser_msg2 = Msg2::to_serde_json_buf(msg2).unwrap();

    // Deserialize to Msg1, this should fail!
    let deser_msg1 = Msg1::from_serde_json_buf(&ser_msg2);
    match deser_msg1 {
        Some(_) => println!("test test_to_from_serde_failure: Unexpected success, `Msg1::from_serde_json_str(&ser_msg2)` should fail as id is MSG2_ID not MSG1_ID"),
        None => panic!("test test_to_from_serde_failure: Expected failure, `Msg1::from_serde_json_str(&ser_msg2)` did fail as id is MSG2_ID not MSG1_ID"),
    }
}
