# Experiment with inter-process channels

Determine the feasibility of combining channels and
network connections to send messages between processes.

Currently just a couple messages and a couple
state machines with two states.

## Run

```
wink@3900x 23-01-23T01:03:28.975Z:~/prgs/rust/myrepos/exper_inter_process_channel (main)
$ cargo run
   Compiling sm_channel_to_network v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/sm_channel_to_network)
   Compiling exper_inter_process_channel v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/exper_inter_process_channel`
main:+
c2n=SmChannelToNetwork { name: c2n, state_info_hash: {0x55632bb4bc20: StateInfo { name: "state0" }, 0x55632bb4bff0: StateInfo { name: "state1" }}; current_state: state0 }
hdr: MsgHeader { id: 0 }
msg1: Msg1 { header: MsgHeader { id: 1 } }
msg1: Msg2 { header: MsgHeader { id: 2 } }
State0: Msg1 { header: MsgHeader { id: 1 } }
State1: Msg2 { header: MsgHeader { id: 2 } }
State0: Msg2 { header: MsgHeader { id: 2 } }
State1: Msg1 { header: MsgHeader { id: 1 } }
n2c=SmNetworkToChannel { name: n2c, state_info_hash: {0x55632bb3bba0: StateInfo { name: "state0" }, 0x55632bb3bf70: StateInfo { name: "state1" }}; current_state: state0 }
State0: Msg1 { header: MsgHeader { id: 1 } }
State1: Msg2 { header: MsgHeader { id: 2 } }
State0: Msg2 { header: MsgHeader { id: 2 } }
State1: Msg1 { header: MsgHeader { id: 1 } }
main:-
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
