Rust bgpq3
==========

[![crates.io](https://img.shields.io/crates/v/bgpq3.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/bgpq3)
[![Released API docs](https://img.shields.io/badge/docs.rs-bgpq3-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/bgpq3)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](./LICENSE)
[![Github](https://img.shields.io/badge/github-rappet/rust--bgpq3-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/rappet/rust_bgpq3)

This library provides a thing wrapper around the [`bgpq3`]/[`bgpq4`] binary.

## Example

bgpq3 AS-Set

```rust
extern crate bgpq3;

pub fn main() {
    let networks = bgpq3::Bgpq3::new().query_v6("AS-RAPPET").unwrap();
    println!("{:?}", networks);
}
```

bgpq3 ASN

```rust
extern crate bgpq3;

pub fn main() {
    let networks = bgpq3::Bgpq3::new().query_v6(207968).unwrap();
    println!("{:?}", networks);
}
```

bgpq4

```rust
extern crate bgpq3;
use bgpq3::Bgpq3;

pub fn main() {
    let networks = Bgpq3::bgpq4().query_v6("AS-RAPPET").unwrap();
    println!("{:?}", networks);
}
```

## Feature flags

- `tokio` for async process invocation.

[`bgpq3`]: https://github.com/snar/bgpq3
[`bgpq4`]: https://github.com/bgp/bgpq4