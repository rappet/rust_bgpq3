//! [<img alt="github" src="https://img.shields.io/badge/github-rappet/rust__bgpq3-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rappet/rust_bgpq3)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/bgpq3.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bgpq3)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bgpq3-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/bgpq3)
//!
//! This library provides a thing wrapper around the [`bgpq3`]/[`bgpq4`] binary.
//!
//! ## Example
//!
//! ```rust
//! extern crate bgpq3;
//!
//! pub fn main() {
//!     let networks = bgpq3::Bgpq3::new().query_v6("AS-RAPPET").unwrap();
//!     println!("{:?}", networks);
//! }
//! ```
//!
//! [`bgpq3`]: https://github.com/snar/bgpq3
//! [`bgpq4`]: https://github.com/bgp/bgpq4

extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate ipnetwork;

use thiserror::Error;
use ipnetwork::{Ipv4Network, Ipv6Network, IpNetworkError};
use serde::Deserialize;
use std::process::Command;

#[derive(Default, Clone)]
pub struct Bgpq3 {
    settings: Bgpq3Settings,
}

impl Bgpq3 {
    pub fn new() -> Bgpq3 {
        Default::default()
    }

    pub fn with_settings(settings: &Bgpq3Settings) -> Bgpq3 {
        Bgpq3 {
            settings: settings.clone(),
        }
    }

    pub fn query_v4(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv4Network>> {
        let output = Command::new(self.settings.version.bin_name())
            // JSON
            .arg("-j")
            // AS-SET
            .arg(query.into().query_string)
            .output()?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;

        let networks: Result<Vec<Ipv4Network>, IpNetworkError> = deserialized.nn
            .into_iter()
            .map(|inner| inner.prefix.parse())
            .collect();
        Ok(networks?)
    }

    pub fn query_v6(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv6Network>> {
        let output = Command::new(self.settings.version.bin_name())
            // JSON
            .arg("-j")
            // IPv6
            .arg("-6")
            // AS-SET
            .arg(query.into().query_string)
            .output()?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;

        let networks: Result<Vec<Ipv6Network>, IpNetworkError> = deserialized.nn
            .into_iter()
            .map(|inner| inner.prefix.parse())
            .collect();
        Ok(networks?)
    }
}

#[derive(Deserialize)]
struct Bgpq3Output {
    #[serde(rename = "NN")]
    nn: Vec<Bgpq3OutputInner>,
}

#[derive(Deserialize)]
struct Bgpq3OutputInner {
    prefix: String,
}

#[derive(Debug, Clone)]
pub struct Bgpq3Query {
   pub(crate) query_string: String,
}

impl Bgpq3Query {
    pub fn as_set(as_set: &str) -> Bgpq3Query {
        Bgpq3Query {
            query_string: as_set.to_string(),
        }
    }

    pub fn asn(asn: u32) -> Bgpq3Query {
        Bgpq3Query {
            query_string: format!("{}", asn.to_string()),
        }
    }
}

impl From<&str> for Bgpq3Query {
    fn from(as_set: &str) -> Self {
        Bgpq3Query::as_set(as_set)
    }
}

impl From<u32> for Bgpq3Query {
    fn from(asn: u32) -> Self {
        Bgpq3Query::asn(asn)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Bgpq3Settings {
    version: Version,
}

impl Bgpq3Settings {
    pub fn new() -> Bgpq3Settings {
        Default::default()
    }
}

#[derive(Debug, Error)]
pub enum Bgpq3Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("bgpq3/bgpq4 did not run successfully")]
    StatusNotSuccess,
    #[error("failed parsing output json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed parsing ip-network: {0}")]
    IpNetwork(#[from] IpNetworkError),
}

impl From<Bgpq3Error> for std::io::Error {
    fn from(err: Bgpq3Error) -> Self {
        use std::io::{Error, ErrorKind};

        match err {
            Bgpq3Error::Io(io) => io,
            Bgpq3Error::StatusNotSuccess => Error::new(ErrorKind::Other, err.to_string()),
            Bgpq3Error::Json(err) => err.into(),
            Bgpq3Error::IpNetwork(_) => Error::new(ErrorKind::InvalidData, err.to_string()),
        }
    }
}

pub type Bgpq3Result<T> = Result<T, Bgpq3Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Version {
    Bgpq3,
    Bgpq4,
}

impl Version {
    pub fn bin_name(self) -> &'static str {
        match self {
            Version::Bgpq3 => "bgpq3",
            Version::Bgpq4 => "bgpq4"
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::Bgpq3
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
