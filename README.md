# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

Currently just a couple messages and a couple
state machines with two states.

## Run

```
wink@3900x 23-02-01T17:38:03.312Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo run
   Compiling msg_header v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_header)
   Compiling msg_serde_json v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_serde_json)
   Compiling msg_macro v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_macro)
   Compiling msg1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg1)
   Compiling msg2 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg2)
   Compiling sm_network_to_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_network_to_channel)
   Compiling sm_channel_to_network v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_channel_to_network)
   Compiling exper_inter_process_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel)
    Finished dev [unoptimized + debuginfo] target(s) in 1.09s
     Running `target/debug/exper_inter_process_channel`
main:+
tickle_ipchnl:+
c2n_ipchnl:+
c2n_ipchnl: Waiting  msg
tickle_ipchnl: inter_process_channel is READY
c2n_ipchnl: Received msg
c2n:State0: Msg1 { header: MsgHeader { id: a88ba7e7-0930-4df6-bb24-240338bf8eb5 }, fu64: 1311768467463790321 }
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
ipchnlr stream: msg_len=83
ipchnlr stream: msg_buf={"header":{"id":"a88ba7e7-0930-4df6-bb24-240338bf8eb5"},"fu64":1311768467463790321}
msg1=Msg1 { header: MsgHeader { id: a88ba7e7-0930-4df6-bb24-240338bf8eb5 }, fu64: 1311768467463790321 }
ipchnlr stream: msg_len=56
ipchnlr stream: msg_buf={"header":{"id":"4029b3c4-f380-488a-8560-8320cc8fb76e"}}
msg2=Msg2 { header: MsgHeader { id: 4029b3c4-f380-488a-8560-8320cc8fb76e } }
tickle_ipchnlr:-
main:-
ipchnlr stream: stream closed reading msg_len, stopping
ipchnlr stream:-
```

## Tests

A few tests
```
wink@3900x 23-02-01T17:38:14.142Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo test --all
   Compiling msg_header v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_header)
   Compiling msg_serde_json v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_serde_json)
   Compiling msg1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg1)
   Compiling msg_macro v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg_macro)
   Compiling msg2 v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/msg2)
   Compiling sm_channel_to_network v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_channel_to_network)
   Compiling sm_network_to_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_network_to_channel)
   Compiling exper_inter_process_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests src/main.rs (target/debug/deps/exper_inter_process_channel-2dc729b6add59341)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (target/debug/deps/test-a0c13e144631c349)

running 1 test
test test_identical_json - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg1-04621d0047accf46)

running 3 tests
test test::test_msg1_to_from_serde_json_buf ... ok
test test::test_hash_map_to_from_serde_json_buf ... ok
test test::test_msg1_to_from_serde_json_str ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg2-386ff9ee3adc9f97)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_header-51e3989e01f22a0d)

running 1 test
test test::test_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_macro-3ed0087914ee585e)

running 3 tests
test test::test_msg_a_from_json_str ... ok
test test::test_msg_a_serde ... ok
test test::test_msg_a_to_from_serde_json_buf ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/msg_macro-0bde7790fd26a42e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_serde_json-6c3b9b67d59ab0ed)

running 3 tests
test test::test_cmp_serde_json_msg_header_with_bad_msg_header ... ok
test test::test_cmp_str_id_an_serde_json_msg_header ... ok
test test::test_cmp_str_id_and_serde_json_msg_header_with_short_id_in_header ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm-6ed153d4965c7216)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_channel_to_network-6fc8532fc6643cbf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_network_to_channel-6d8733a4dfa7844f)

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

   Doc-tests msg_macro

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg_serde_json

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
