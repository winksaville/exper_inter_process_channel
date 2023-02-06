# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

## Run

Currently `main()` is not doing anything:

```
wink@3900x 23-02-06T16:50:00.844Z:~/prgs/rust/myrepos/exper_inter_process_channel (wip-ipchnl-between-client-server)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/exper_inter_process_channel`
main:+
main:-
```

## Tests

A few tests
```
wink@3900x 23-02-06T16:49:57.598Z:~/prgs/rust/myrepos/exper_inter_process_channel (wip-ipchnl-between-client-server)
$ cargo test --all
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/actor-300d4142e8742c01)

running 1 test
test test::test_actor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/client-5211ecf4cabe1077)

running 2 tests
test test::test_ping_count_0 ... ok
test test::test_ping_counts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_complete-24a6d85cc6972ec8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_protocol-7395e4bf6553eb53)

running 2 tests
test test::test_default_echo_protocol ... ok
test test::test_echo_protocol ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_reply-6fd22f312f4dbc21)

running 1 test
test test::test_echo_reply_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_req-c0bb87d168ce087d)

running 1 test
test test::test_echo_req_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/echo_start-9e04301947a8df42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_inter_process_channel-0830f2a6f9779e11)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (target/debug/deps/test-19c6ddbdaf3d7fb6)

running 1 test
test test_identical_json - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg1-affaf4e7fb649a9b)

running 2 tests
test test::test_msg1_to_from_serde_json_buf ... ok
test test::test_hash_map_to_from_serde_json_buf ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg2-e76708067b805ead)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_header-10ddeaa2fe520bcb)

running 2 tests
test test::test_id ... ok
test test::test_msg_id_utf8_len ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_local_macro-b1b0eb4b2d9a8218)

running 1 test
test test::test_with_fields_including_a_sender ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_serde_json-f1e1353c35d8b2eb)

running 1 test
test test::test_get_id_utf8_str ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/msg_serde_macro-aaef67206961e50c)

running 2 tests
test test::test_msg_a_to_from_serde_json_buf ... ok
test test::test_with_fields ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/protocol-7f9e7e5795995782)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/server-ca25e34559c2f599)

running 1 test
test test::test_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests actor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests client

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_complete

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_reply

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_req

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_start

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

   Doc-tests msg_serde_json

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests msg_serde_macro

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests server

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Performance

A simple test of the performance of simulating a client and using and `server`.
Both running in main thread. This is an "optimal" setup and tests in the future will see
what the "actual" results are. I'm also running the test on CPU 0 using `taskset -c 0`
which produces consistent and lower times.

Also, note that the first time through the loop is always very slow! In this test
I run the loop 100 times, print the first and last 5 recorded entries and the `Average
times of the last 80 loops`.

```
wink@3900x 23-02-06T16:52:07.702Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (wip-ipchnl-between-client-server)
$ taskset -c 0 cargo test --release -- --nocapture
    Finished release [optimized] target(s) in 0.03s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/release/deps/server-5b6c301d4a714896)

running 1 test
test test::test_1 ... test_1: server=server { name: server, state_info_hash: {0x55e95826a7b0: StateInfo { name: "state0" }}; current_state: state0 }
test_1:          second_now_ns - first_now_ns =    190ns
test_1:          third_now_ns - second_now_ns =     60ns

First loop
  t0 =    161ns
  t1 =   1953ns
  t2 =    351ns
 rtt =   2465ns

Loop 96
  t0 =     40ns
  t1 =     91ns
  t2 =     90ns
 rtt =    221ns

Loop 97
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 98
  t0 =     40ns
  t1 =    101ns
  t2 =     90ns
 rtt =    231ns

Loop 99
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 100
  t0 =     40ns
  t1 =     91ns
  t2 =    100ns
 rtt =    231ns

Average times of the last 80 loops
  t0 = 41ns
  t1 = 92ns
  t2 = 95ns
 rtt = 228ns
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
