use actor_executor::{
    add_actor_to_actor_executor_blocking, initialize_supervisor_con_mgr_actor_executor_blocking,
};
use an_id::AnId;
use cmd_done_issuer_protocol::CmdDone;
use echo_requestee_protocol::{EchoReq, ECHO_REQ_ID};
use insert_key_msg_id_value_from_serde_json_buf_requester_protocol::{
    InsertKeyMsgIdValueFromSerdeJsonBufReq, InsertKeyMsgIdValueFromSerdeJsonBufRsp,
    InsertKeyMsgIdValueFromSerdeJsonBufRspStatus,
};
use insert_key_msg_id_value_to_serde_json_buf_requester_protocol::{
    InsertKeyMsgIdValueToSerdeJsonBufReq, InsertKeyMsgIdValueToSerdeJsonBufRsp,
    InsertKeyMsgIdValueToSerdeJsonBufRspStatus,
};
use msg_router_dispatcher::MsgRouterDispatcher;
use msg_router_forwarder::MsgRouterForwarder;
use msg_router_forwarder_actor_sender_requester_protocol::{
    MsgRouterForwarderActorSenderReq, MsgRouterForwarderActorSenderRsp,
};
use sender_map_by_instance_id::sender_map_get;

#[test]
fn test_msg_router_forwarder_dispatcher() {
    println!("\ntest_msg_router_forwarder_dispatcher:+");

    // Initialize Supervisor starting a single ActorExecutor and the connection manager
    println!("test_msg_router_forwarder_dispatcher: create supervisor, con_mgr and ae");
    let (
        supervisor_instance_id,
        supervisor_chnl,
        ae_join_handle,
        ae_instance_id,
        _con_mgr_instance_id,
    ) = initialize_supervisor_con_mgr_actor_executor_blocking();
    let ae_sender = sender_map_get(&ae_instance_id).unwrap();

    // Add MsgRouterDispatcher to ActorExecutor
    println!("test_msg_router_forwarder_dispatcher: add MsgRouterDispatcher to ae");
    let addr = "127.0.0.1:12345";
    let mrd1 = Box::new(MsgRouterDispatcher::new("mrd1", &addr));
    let (_mrd1_actor_id, mrd1_instance_id) = add_actor_to_actor_executor_blocking(
        mrd1,
        &ae_instance_id,
        &supervisor_instance_id,
        &supervisor_chnl.receiver,
    );

    // Add MsgRouterForwarder to ActorExecutor
    println!("test_msg_router_forwarder_dispatcher: add MsgRouterForward to ae");
    let mrf1 = Box::new(MsgRouterForwarder::new("mrf1", addr));
    let (_mrf1_actor_id, mrf1_instance_id) = add_actor_to_actor_executor_blocking(
        mrf1,
        &ae_instance_id,
        &supervisor_instance_id,
        &supervisor_chnl.receiver,
    );

    // Add EchoReq::to_serde_json_buf to forwarder
    let msg = Box::new(InsertKeyMsgIdValueToSerdeJsonBufReq::new(
        &mrf1_instance_id,
        &supervisor_instance_id,
        &ECHO_REQ_ID,
        EchoReq::to_serde_json_buf,
    ));
    sender_map_get(&mrf1_instance_id)
        .unwrap()
        .send(msg)
        .unwrap();

    println!(
        "test_msg_router_forwarder_dispatcher: waiting for InsertKeyMsgIdValueToSerdeJsonBufRsp"
    );
    let msg_any = supervisor_chnl.receiver.recv().unwrap();
    let msg = InsertKeyMsgIdValueToSerdeJsonBufRsp::from_box_msg_any(&msg_any).unwrap();
    println!("test_msg_router_forwarder_dispatcher: msg={:?}", msg);
    assert_eq!(
        msg.status,
        InsertKeyMsgIdValueToSerdeJsonBufRspStatus::Success
    );

    // Add EchoReq::from_serde_json_buf to dispatcher
    let msg = Box::new(InsertKeyMsgIdValueFromSerdeJsonBufReq::new(
        &mrd1_instance_id,
        &supervisor_instance_id,
        &ECHO_REQ_ID,
        EchoReq::from_serde_json_buf,
    ));
    sender_map_get(&mrd1_instance_id)
        .unwrap()
        .send(msg)
        .unwrap();

    println!(
        "test_msg_router_forwarder_dispatcher: waiting for InsertKeyMsgIdValueFromSerdeJsonBufRsp"
    );
    let msg_any = supervisor_chnl.receiver.recv().unwrap();
    let msg = InsertKeyMsgIdValueFromSerdeJsonBufRsp::from_box_msg_any(&msg_any).unwrap();
    println!("test_msg_router_forwarder_dispatcher: msg={:?}", msg);
    assert_eq!(
        msg.status,
        InsertKeyMsgIdValueFromSerdeJsonBufRspStatus::Success
    );

    println!("test_msg_router_forwarder_dispatcher: send MsgRouterForwarderActorSenderReq");
    let msg = Box::new(MsgRouterForwarderActorSenderReq::new(
        &mrf1_instance_id,
        &supervisor_instance_id,
        &AnId::nil(), // Currently not used
    ));
    sender_map_get(&mrf1_instance_id)
        .unwrap()
        .send(msg)
        .unwrap();

    println!("test_msg_router_forwarder_dispatcher: wait for MsgRouterForwarderActorSenderRsp");
    let msg_any = supervisor_chnl.receiver.recv().unwrap();
    let msg = MsgRouterForwarderActorSenderRsp::from_box_msg_any(&msg_any).unwrap();
    println!("test_msg_router_forwarder_dispatcher: msg={:?}", msg);
    let forwarder = &msg.sender;

    // Send an EchoReq from us, the supervisor, through forwarder in MsgRouterForwarder
    // which will transfer it to MsgRouterDispatcher and then back to us, the supervisor.
    println!("test_msg_router_forwarder_dispatcher: send EchoReq to mrf1_forwarder");
    let msg = Box::new(EchoReq::new(
        &supervisor_instance_id,
        &supervisor_instance_id,
        123,
    ));
    forwarder.send(msg).unwrap();

    println!("test_msg_router_forwarder_dispatcher: wait for EchoReq from dispatcher");
    let msg_any = supervisor_chnl.receiver.recv().unwrap();
    let msg = EchoReq::from_box_msg_any(&msg_any).unwrap();
    println!("test_msg_router_forwarder_dispatcher: msg={:?}", msg);
    assert_eq!(msg.counter, 123);

    println!("test_msg_router_forwarder_dispatcher: send CmdDone to ae");
    let msg = Box::new(CmdDone::new(&ae_instance_id, &supervisor_instance_id));
    ae_sender.send(msg).unwrap();
    println!("test_msg_router_forwarder_dispatcher: sent ae CmdDone");

    println!("test_msg_router_forwarder_dispatcher: join ae");
    ae_join_handle.join().unwrap();
    println!("test_multiple_ae: join ae has completed");

    println!("test_msg_router_forwarder_dispatcher:-");
}
