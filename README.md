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

```
wink@3900x 23-02-08T01:35:09.492Z:~/prgs/rust/myrepos/exper_inter_process_channel/manager (actors-and-managers)
$ cargo test --all
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/actor-a0b12ccd3e1040c4)

running 1 test
test test::test_actor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/client-61913046b55346aa)

running 2 tests
test test::test_ping_count_0 ... ok
test test::test_ping_counts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_complete-3fc31743a291ff0f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_reply-7583f388b9983c48)

running 1 test
test test::test_echo_reply_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_req-a9234b4d98d17033)

running 1 test
test test::test_echo_req_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_req_reply_protocol-69c77722a7020dc6)

running 1 test
test test::test_echo_req_reply_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_start-e731b54bdd688d2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_start_complete_protocol-5265635d2722b89a)

running 1 test
test test::test_echo_start_complete_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/exper_inter_process_channel-1484523759a67dd4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/test-78078b3bf19a6f80)

running 1 test
test test_identical_json - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/manager-e28b3f5110d163fb)

running 1 test
test test::test_manager ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg1-f4c7bbe957f930e0)

running 2 tests
test test::test_hash_map_to_from_serde_json_buf ... ok
test test::test_msg1_to_from_serde_json_buf ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg2-73da51187cca0e3f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_header-c705ad8ab8c93ac1)

running 2 tests
test test::test_msg_id_utf8_len ... ok
test test::test_id ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_local_macro-8e120248175c6c0d)

running 1 test
test test::test_with_fields_including_a_sender ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_serde_json-f6b2e7b6642e9921)

running 1 test
test test::test_get_id_utf8_str ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_serde_macro-ccba21a65bc9d162)

running 2 tests
test test::test_with_fields ... ok
test test::test_msg_a_to_from_serde_json_buf ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/protocol-1445e700e9aa0762)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/protocol_set-ce61c81173b91a66)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/server-fa394cff9ebdb056)

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

   Doc-tests echo_reply

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_req

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_req_reply_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_start

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests echo_start_complete_protocol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests manager

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

   Doc-tests protocol_set

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
wink@3900x 23-02-08T01:36:30.117Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (actors-and-managers)
$ taskset -c 0 cargo test --release -- --nocapture
    Finished release [optimized] target(s) in 0.03s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/release/deps/server-0c8ebcb2c5ca6929)

running 1 test
test test::test_1 ... test_1: server=server { name: server, state_info_hash: {0x557338f8cfc0: StateInfo { name: "state0" }}; current_state: state0 }
test_1:          second_now_ns - first_now_ns =    471ns
test_1:          third_now_ns - second_now_ns =     50ns

First loop
  t0 =     70ns
  t1 =    912ns
  t2 =    341ns
 rtt =   1323ns

Loop 96
  t0 =     40ns
  t1 =    101ns
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
  t2 =    100ns
 rtt =    231ns

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
