# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

Currently just a couple messages and a couple
state machines with two states.

## Run

```
wink@3900x 23-01-25T22:00:27.347Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_inter_process_channel`
main:+
tickle_ipchnl:+
c2n_ipchnl:+
c2n_ipchnl: Waiting  msg
tickle_ipchnl: inter_process_channel is READY
c2n_ipchnl: Received msg
c2n:State0: Msg1 { header: MsgHeader { id: a88ba7e7-0930-4df6-bb24-240338bf8eb5 } }
c2n_ipchnl: Waiting  msg
tickle_ipchnl: completed msg1
c2n_ipchnl: Received msg
c2n:State1: Msg2 { header: MsgHeader { id: 4029b3c4-f380-488a-8560-8320cc8fb76e } }
c2n_ipchnl: Waiting  msg
tickle_ipchnl: completed msg2
tickle_ipchnl:-
tickle_ipchnlr:+
c2n_ipchnl:-
ipchnlr:+
ipchnlr stream:+
ipchnlr stream: msg_len=56
ipchnlr stream: msg_buf={"header":{"id":"a88ba7e7-0930-4df6-bb24-240338bf8eb5"}}
msg1=Msg1 { header: MsgHeader { id: a88ba7e7-0930-4df6-bb24-240338bf8eb5 } }
ipchnlr stream: msg_len=56
ipchnlr stream: msg_buf={"header":{"id":"4029b3c4-f380-488a-8560-8320cc8fb76e"}}
msg2=Msg2 { header: MsgHeader { id: 4029b3c4-f380-488a-8560-8320cc8fb76e } }
tickle_ipchnlr:-
ipchnlr stream: stream closed reading msg_len, stopping
ipchnlr stream:-
main:-
```

## Tests

A few tests
```
wink@3900x 23-01-25T21:59:28.013Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo test --all
   Compiling msg_header v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_header)
   Compiling msg2 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg2)
   Compiling msg1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg1)
   Compiling sm_channel_to_network v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_channel_to_network)
   Compiling sm_network_to_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_network_to_channel)
    Finished test [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests src/main.rs (target/debug/deps/exper_inter_process_channel-b9fe436f5605ed4f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (target/debug/deps/test-13eb2bba3f09a276)

running 1 test
test test_identical_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg1-24c4ff0908757d53)

running 3 tests
test test::test_msg1_from_json_buf ... ok
test test::test_msg1_from_json_str ... ok
test test::test_msg1_serde ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg2-b807e8cb805ce9ac)

running 3 tests
test test::test_msg2_from_json_buf ... ok
test test::test_msg2_from_json_str ... ok
test test::test_msg2_serde ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_header-51e3989e01f22a0d)

running 4 tests
test test::test_cmp_serde_json_msg_header_with_bad_msg_header ... ok
test test::test_cmp_str_id_an_serde_json_msg_header ... ok
test test::test_cmp_str_id_and_serde_json_msg_header_with_short_id_in_header ... ok
test test::test_id ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm-6ed153d4965c7216)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_channel_to_network-8a75b77cd8a1cbe1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_network_to_channel-17f364f92e479f83)

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

   Doc-tests sm

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sm_channel_to_network

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sm_network_to_channel

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
