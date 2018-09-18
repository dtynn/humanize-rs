### humanize-rs
this is a rust port of [github.com/dustin/go-humanize](https://github.com/dustin/go-humanize).

#### Bytes
```
use humanize_rs::bytes::{Bytes, Unit};

let gigabytes1 = Bytes::new(1, Unit::GiByte);
let gigabytes2 = "1 GiB".parse::<Bytes>();
assert_eq!(gigabytes1, gigabytes2);
```
