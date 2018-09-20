## humanize-rs
[![Crates.io](https://img.shields.io/crates/v/humanize-rs.svg)](https://crates.io/crates/humanize-rs)
[![Docs](https://docs.rs/humanize-rs/badge.svg)](https://docs.rs/humanize-rs)

This lib aims at converting human-readable strings to specific types.
It's mainly used in parsing config files.

#### Bytes
```
use humanize_rs::bytes::{Bytes, Unit};

let gigabytes1 = Bytes::new(1, Unit::GiByte);
let gigabytes2 = "1 GiB".parse::<Bytes>();
assert_eq!(gigabytes1, gigabytes2);
```

#### Duration
```
use humanize_rs::duration::parse;
use std::time::Duration;

assert_eq!(parse("1h 30m 71s"), Ok(Duration::from_secs(60 * 90 + 71)));
```
