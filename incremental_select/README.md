# Experiment incrementally adding actors to an executor

In my model for a Rust implemenation of the actor model
actors need to communicate with each other. For that I
will use channels and each connection will be independent.
Further more, for efficiency, mulitple actors can share
a thread. To accomplish these goals I need a communication
mechanism where a single thread can "wait" on multiple
channels. This cannot be accomplished with the Rust
`std::sync::mpsc::channel`. Instead, I've chosen to use
`crossbeam_channel`s.

In particular I'm using
[`Select`](https://docs.rs/crossbeam/latest/crossbeam/channel/struct.Select.html)
which supports selecting a ready channel from a set of channels.

In all of the [`examples`](https://docs.rs/crossbeam/latest/crossbeam/channel/struct.Select.html)
I found the set of operations, `Receivers<_>` in this case
is created first and then added enmass to `Select` using the
`recv` method.

But I want to be able to add actors incrementally and potentially
move actors between threads. Initial attempt:
```rust
fn simple() {
    use crossbeam_channel::{Receiver, Select, Sender, unbounded};

    let mut channels: Vec::<(Sender<i32>, Receiver<i32>)> = Vec::new();
    let mut sel = Select::new();

    for i in 0..=1 {
        let (tx, rx) = unbounded::<i32>();
        channels.push((tx, rx));
        sel.recv(&channels[i].1);
    }
}
```

You can run this via:
```bash
wink@fwlaptop 23-02-25T20:44:36.753Z:~/prgs/rust/myrepos/exper_inter_process_channel/incremental_select (add-incremental_select)
$ cargo run --features "simple"
   Compiling incremental_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_inter_process_channel/incremental_select)
error[E0502]: cannot borrow `channels` as mutable because it is also borrowed as immutable
  --> incremental_select/src/main.rs:10:9
   |
10 |         channels.push((tx, rx));
   |         ^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
11 |         sel.recv(&channels[i].1);
   |         ------------------------
   |         |         |
   |         |         immutable borrow occurs here
   |         immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `incremental_select` due to previous error```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
