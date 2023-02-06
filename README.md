# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

## Run

Currently `main()` is not doing anything:

```
wink@3900x 23-02-06T02:11:24.196Z:~/prgs/rust/myrepos/exper_inter_process_channel (wip-ipchnl-between-client-server)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/exper_inter_process_channel`
main:+
main:-
```

## Tests

A few tests
```
wink@3900x 23-02-06T02:04:52.678Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (wip-ipchnl-between-client-server)
$ cargo test --all
   Compiling echo_req v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/echo_req)
   Compiling echo_start v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/echo_start)
   Compiling echo_reply v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/echo_reply)
   Compiling server v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/server)
   Compiling client v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/client)
   Compiling echo_protocol v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/echo_protocol)
   Compiling exper_inter_process_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/client-a7252bdb3de1d2ef)

running 2 tests
test test::test_ping_count_0 ... ok
test test::test_ping_counts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_complete-24a6d85cc6972ec8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_protocol-7395e4bf6553eb53)

running 2 tests
test test::test_default_echo_protocol ... ok
test test::test_echo_protocol ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_reply-6fd22f312f4dbc21)

running 1 test
test test::test_echo_reply_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_req-c0bb87d168ce087d)

running 1 test
test test::test_echo_req_new ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/echo_start-9e04301947a8df42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/exper_inter_process_channel-2654830f7f7a5479)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/test-57be8fa542d129af)

running 1 test
test test_identical_json - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg1-affaf4e7fb649a9b)

running 2 tests
test test::test_msg1_to_from_serde_json_buf ... ok
test test::test_hash_map_to_from_serde_json_buf ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg2-e76708067b805ead)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_header-10ddeaa2fe520bcb)

running 2 tests
test test::test_id ... ok
test test::test_msg_id_utf8_len ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_local_macro-b1b0eb4b2d9a8218)

running 1 test
test test::test_with_fields_including_a_sender ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_serde_json-f1e1353c35d8b2eb)

running 1 test
test test::test_get_id_utf8_str ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/msg_serde_macro-aaef67206961e50c)

running 2 tests
test test::test_msg_a_to_from_serde_json_buf ... ok
test test::test_with_fields ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/protocol-7f9e7e5795995782)

running 1 test
test test::test_protocol ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/server-a6febba406c5c94f)

running 1 test
test test::test_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/debug/deps/sm-f72748f98b8e6ac2)

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

   Doc-tests sm

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
wink@3900x 23-02-06T16:34:17.436Z:~/prgs/rust/myrepos/exper_inter_process_channel/server (wip-ipchnl-between-client-server)
$ taskset -c 0 cargo test --release -- --nocapture
   Compiling server v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/server)
    Finished release [optimized] target(s) in 1.65s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/target/release/deps/server-7c30956a02c107b8)

running 1 test
test test::test_1 ... test_1: server=server { name: server, state_info_hash: {0x560a8d4427c0: StateInfo { name: "state0" }}; current_state: state0 }
test_1:          second_now_ns - first_now_ns =    231ns
test_1:          third_now_ns - second_now_ns =     50ns

First loop
  t0 =     80ns
  t1 =   1663ns
  t2 =    301ns
 rtt =   2044ns

Loop 96
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 97
  t0 =     41ns
  t1 =     90ns
  t2 =     90ns
 rtt =    221ns

Loop 98
  t0 =     40ns
  t1 =     90ns
  t2 =    100ns
 rtt =    230ns

Loop 99
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Loop 100
  t0 =     40ns
  t1 =     90ns
  t2 =     90ns
 rtt =    220ns

Average times of the last 80 loops
  t0 = 40ns
  t1 = 90ns
  t2 = 94ns
 rtt = 225ns
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
