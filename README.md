# Abondoned effort

This project is abondoned in favour of [broadcast-channel](https://github.com/smol-rs/broadcast-channel).

---

# async-broadcast

While [`async_channel`] provides a nice and simple multi-producer-multi-consumer channel, this
library provides broadcasting feature, where every message sent on the channel is received by
every receiver. Since the ownership of the data is transfered, the data is cloned for each receiver
and hence `Clone` trait is required on the type of the data being transmitted.

[`async_channel`]: https://crates.io/crates/async-channel

### Examples

```rust
let (sender1, receiver1) = async_broadcast::unbounded();
let sender2 = sender1.clone();
let receiver2 = receiver1.clone();

sender1.try_send(1).unwrap();
sender2.try_send(2).unwrap();

assert_eq!(receiver1.try_recv(), Ok(1));
assert_eq!(receiver1.try_recv(), Ok(2));

assert_eq!(receiver2.try_recv(), Ok(1));
assert_eq!(receiver2.try_recv(), Ok(2));
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
