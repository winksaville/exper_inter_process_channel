# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

## Weird Bugs

 * Rust-Analyzer is reporting a false type-mismatch error [#14](https://github.com/winksaville/exper_inter_process_channel/issues/14)
 and [Issue 14475](https://github.com/rust-lang/rust-analyzer/pull/14475), when merged, might fix this.

## Run

Currently `main()` is not doing anything:

```
wink@3900x 23-04-14T17:40:56.988Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/exper_inter_process_channel`
experiment_inter_process_channel, does nothing!
```

## Tests

```
wink@3900x 23-04-14T17:34:21.041Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo test --all
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/actor-d6db87bbaf2429f6)

running 1 test
test test::test_actor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/actor_channel-2b5c1529d893bb9b)

running 1 test
test test::test_actor_channel ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/actor_executor-0688a68693d8f1de)

running 3 tests
test tests::test_con_mgr_server ... ok
test tests::test_con_mgr_client_server ... ok
test tests::test_multiple_ae ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/actor_executor_protocol-b8fa660b45919389)

running 1 test
test test::test_actor_executor_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/an_id-d836fb60e500754e)

running 1 test
test test::test_an_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/box_msg_any-6022a7d50a95d1bb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/client-ac2fb0b3950ed4ef)

running 2 tests
test test::test_cmd_init ... ok
test test::test_client_ping_with_supervisor_as_server ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/cmd_done-f974bdc2cd0dd90c)

running 1 test
test test::test_cmd_done_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/cmd_init-59cb0f0eef957b85)

running 1 test
test test::test_cmd_done_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/cmd_init_protocol-2156dec86cf4d600)

running 1 test
test test::test_cmd_init_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/con_mgr-135527f2eb8fbefe)

running 2 tests
test test::test_con_mgr_ping ... ok
test test::test_reg_client_server ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/con_mgr_query_protocol-16037fe1e18f95b4)

running 1 test
test test::test_con_mgr_query_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/con_mgr_register_actor_protocol-5b0e9b09ffce5d45)

running 2 tests
test test::test_con_mgr_reg_actor_protocol ... ok
test test::test_con_mgr_reg_actor_protocol_set ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_complete-a8a8a75590ea41f4)

running 1 test
test test::test_cmd_done_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_req-a12e037c5a39e8e8)

running 1 test
test test::test_echo_req_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_requestee_protocol-1ec7bf152b64876a)

running 1 test
test test::test_echo_requestee_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_requester_protocol-549ca8023ac5402c)

running 1 test
test test::test_echo_requester_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_rsp-721720f761905188)

running 1 test
test test::test_echo_rsp_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_start-e1483ac493e31e9c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_start_complete_protocol-d6f05fbed21433be)

running 1 test
test test::test_echo_start_complete_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_inter_process_channel-2e3a9d33018dadac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_msg_serde_proof_of_concept.rs (target/debug/deps/test_msg_serde_proof_of_concept-889066a461d5c88a)

running 1 test
test test_msg_serde_proof_of_concept ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_serde_success_failure.rs (target/debug/deps/test_serde_success_failure-60de9245587276b7)

running 2 tests
test test_to_from_serde_failure - should panic ... ok
test test_to_from_serde_success ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/incremental_select-96a8debe1cefaf80)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg1-84ec8b3e8707225e)

running 3 tests
test test::test_msg1_to_from_serde_json_buf ... ok
test test::test_hash_map_to_from_serde_json_buf ... ok
test test::test_hash_map_to_from_serde_json_buf_src_id_none ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg2-4f6baf0475ce940e)

running 1 test
test test::test_cmd_done_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_header-331320b158c4dfb6)

running 4 tests
test get_msg_id_str_from_buf::test::test_get_id_utf8_str ... ok
test test::test_msg_id_utf8_len ... ok
test test::test_size ... ok
test test::test_new ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_local_macro-20f2be0086590533)

running 1 test
test test::test_with_fields_including_a_sender ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_serde_macro-5becb4e25b729a6c)

running 2 tests
test test::test_msg_a_to_from_serde_json_buf ... ok
test test::test_with_fields ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/name_id-3824fe402a43cd0d)

running 2 tests
test test::test_name_id ... ok
test test::test_struct_with_name_id ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/protocol-70082a31e347e175)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/protocol_set-76d918e3969ad9ef)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/req_add_actor-3f5dc5f0c26af534)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/rsp_add_actor-5d74cf7c1ce70d15)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sender_map_by_instance_id-74770278a0e91882)

running 1 test
test test::test_sender_map ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/server-a685cd15a4f3eb1d)

running 2 tests
test test::test_cmd_init ... ok
test test::test_1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests actor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests actor_channel

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests actor_executor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests actor_executor_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests an_id

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests box_msg_any

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests client

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests cmd_done

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests cmd_init

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests cmd_init_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests con_mgr

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests con_mgr_query_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests con_mgr_register_actor_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_complete

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_req

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_requestee_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_requester_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_rsp

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_start

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_start_complete_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg2

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg_header

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg_local_macro

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg_serde_macro

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests name_id

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests protocol_set

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests req_add_actor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rsp_add_actor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sender_map_by_instance_id

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests server

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@3900x 23-04-14T17:34:27.028Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ 
```

## Performance

A simple test of the performance of simulating a client and using and `server`.
Both running in main thread. This is an "optimal" setup and tests in the future will see
what the "actual" results are. I'm also running the test on CPU 0 using `taskset -c 0`
which produces consistent and lower times.

Also, note that the first time through the loop is always very slow! In this test
I run the loop 100 times, print the first and last 5 recorded entries and the `Average
times of the last 80 loops`.

Newest run, slowed down from 220 to 230-240ns:
```
wink@3900x 23-04-14T17:35:31.896Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (main)
$ taskset -c 0 cargo test --release --  --nocapture
   Compiling msg_header v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_header)
   Compiling msg_serde_macro v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_serde_macro)
   Compiling protocol v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/protocol)
   Compiling protocol_set v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/protocol_set)
   Compiling actor v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/actor)
   Compiling echo_req v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msgs/echo_req)
   Compiling echo_rsp v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msgs/echo_rsp)
   Compiling cmd_init v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msgs/cmd_init)
   Compiling msg_local_macro v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_local_macro)
   Compiling con_mgr_register_actor_protocol v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/protocols/con_mgr_register_actor_protocol)
   Compiling cmd_init_protocol v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/protocols/cmd_init_protocol)
   Compiling echo_requestee_protocol v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/protocols/echo_requestee_protocol)
   Compiling server v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/server)
    Finished release [optimized] target(s) in 3.91s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/release/deps/server-932c1c770094300e)

running 2 tests
test test::test_1 ... 
test_1:+
sender_map_insert: instance_id: 9d930403-13fc-4ede-86df-0610c71ca3da
Server::new(server)
sender_map_insert: instance_id: a5f3750c-a1ce-4c55-9c6e-23e8bd0c9126
test_1: server { id: d9a4c51e-c42e-4f2e-ae6c-96f62217d892 instance_id: a5f3750c-a1ce-4c55-9c6e-23e8bd0c9126 state_info_hash: {0x564012a7a920: StateInfo { name: "state0" }}; current_state: state0; protocol_set: ProtocolSet { name: "server_ps", id: 4c797cb5, protocols_map: {16e9c5a6: Protocol { name: "echo_requestee_protocol", id: 16e9c5a6, messages: [ada0f9a9, 8206e26f] }, 151ae493: Protocol { name: "cmd_init_protocol", id: 151ae493, messages: [16119f69] }} }}
test_1:          second_now_ns - first_now_ns =    180ns
test_1:          third_now_ns - second_now_ns =     60ns

First loop
  t0 =     70ns
  t1 =   1082ns
  t2 =    301ns
 rtt =   1453ns

Loop 6
  t0 =     40ns
  t1 =     90ns
  t2 =    100ns
 rtt =    230ns

Loop 7
  t0 =     40ns
  t1 =     90ns
  t2 =    100ns
 rtt =    230ns

Loop 8
  t0 =     40ns
  t1 =    100ns
  t2 =     90ns
 rtt =    230ns

Loop 9
  t0 =     40ns
  t1 =    100ns
  t2 =    100ns
 rtt =    240ns

Loop 10
  t0 =     50ns
  t1 =     90ns
  t2 =    100ns
 rtt =    240ns

Average times of the last 8 loops
  t0 = 42ns
  t1 = 95ns
  t2 = 97ns
 rtt = 235ns
test_1:-
ok
test test::test_cmd_init ... 
test_cmd_init:+
sender_map_insert: instance_id: 4c36d767-d8d0-41d8-a0d3-f770f7a9c00c
Server::new(server)
sender_map_insert: instance_id: 174ebac8-145c-4b9a-a6c3-34f880e6bdf7
server:State0: CmdInit { header: mh { msg_id: 16119f69 dst_id: 174ebac8 src_id: 4c36d767 } }
server:State0: sending ConMgrRegisterActorReq=ConMgrRegisterActorReq { header: mh { msg_id: b0e83356 dst_id: 4c36d767 src_id: 174ebac8 }, name: "server", id: d9a4c51e, instance_id: 174ebac8, protocol_set: ProtocolSet { name: "server_ps", id: 4c797cb5, protocols_map: {16e9c5a6: Protocol { name: "echo_requestee_protocol", id: 16e9c5a6, messages: [ada0f9a9, 8206e26f] }, 151ae493: Protocol { name: "cmd_init_protocol", id: 151ae493, messages: [16119f69] }} } }server:State0: ConMgrRegisterActorRsp { header: mh { msg_id: db6a401d dst_id: 174ebac8 src_id: 4c36d767 }, status: Success }
test_cmd_init:-
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests server

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@3900x 23-04-14T17:35:52.694Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (main)
```

Older run:
```
wink@3900x 23-02-08T18:06:11.238Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (main)
$ taskset -c 0 cargo test --release -- --nocapture
    Finished release [optimized] target(s) in 0.03s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/release/deps/server-73d8941c82ebb4b1)

running 1 test
test test::test_1 ... test_1: server=server { name: server, state_info_hash: {0x55a63e922fc0: StateInfo { name: "state0" }}; current_state: state0 }
test_1:          second_now_ns - first_now_ns =    271ns
test_1:          third_now_ns - second_now_ns =     50ns

First loop
  t0 =     70ns
  t1 =   1333ns
  t2 =    290ns
 rtt =   1693ns

Loop 96
  t0 =     50ns
  t1 =     91ns
  t2 =     90ns
 rtt =    231ns

Loop 97
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 98
  t0 =     40ns
  t1 =     91ns
  t2 =     90ns
 rtt =    221ns

Loop 99
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 100
  t0 =     40ns
  t1 =     91ns
  t2 =     90ns
 rtt =    221ns

Average times of the last 80 loops
  t0 = 40ns
  t1 = 92ns
  t2 = 94ns
 rtt = 227ns
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests server

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
