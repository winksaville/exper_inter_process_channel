use an_id::AnId;
use msg1::Msg1;
use msg2::Msg2;

#[test]
#[should_panic]
fn test_identical_json() {
    let supervisor_instance_id = AnId::new();

    // Create a Box<Msg2>
    let msg2 = Box::new(Msg2::new(&supervisor_instance_id));
    let ser_msg2 = Msg2::to_serde_json_buf(msg2).unwrap();

    // Deserialize to Msg1, this should fail
    let deser_bad_msg2 = Msg1::from_serde_json_buf(&ser_msg2);
    match deser_bad_msg2 {
        Some(_) => {
            println!("test test_identical_json: Unexpected success, `Msg1::from_serde_json_str(&ser_msg2)` should fail as id is MSG2_ID not MSG1_ID");
        }
        None => panic!("This is expected"),
    }
}
