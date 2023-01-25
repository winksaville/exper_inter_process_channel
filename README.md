# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

Currently just a couple messages and a couple
state machines with two states.

## Run

```
wink@3900x 23-01-25T00:35:55.333Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo run
   Compiling exper_inter_process_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/exper_inter_process_channel`
main:+
c2n_ipchnl:+
c2n_ipchnl: Waiting  msg
main: inter_process_channel is READY
c2n_ipchnl: Received msg
c2n:State0: Msg1 { header: MsgHeader { id: a88ba7e7-0930-4df6-bb24-240338bf8eb5 } }
c2n_ipchnl: Waiting  msg
main: completed msg1
c2n_ipchnl: Received msg
c2n:State1: Msg2 { header: MsgHeader { id: 4029b3c4-f380-488a-8560-8320cc8fb76e } }
c2n_ipchnl: Waiting  msg
main: completed msg2
inter_process_channel_reciver:+
inter_process_channel_reciver stream:+
inter_process_channel_reciver stream: msg_len_buf=[4, 0]
inter_process_channel_reciver stream: msg_len=4
inter_process_channel_reciver stream: msg_buf=[1, 2, 3, 4]
main:-
inter_process_channel_reciver stream: stream closed reading msg_len, stopping
inter_process_channel_reciver stream:-
c2n_ipchnl:-
```

## Tests

A few tests
```
wink@3900x 23-01-25T00:20:51.398Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo test --all
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs (target/debug/deps/exper_inter_process_channel-8d5194b92dbbae52)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (target/debug/deps/test-1ac436d00ca818e3)

running 1 test
test test_identical_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg1-24c4ff0908757d53)

running 1 test
test test::test_msg1_serde ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg2-b807e8cb805ce9ac)

running 1 test
test test::test_msg2_serde ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_header-51e3989e01f22a0d)

running 1 test
test test::test_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm-6ed153d4965c7216)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_channel_to_network-80abdf6542593f97)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/sm_network_to_channel-c663c78e9235f995)

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
