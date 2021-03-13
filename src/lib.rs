//! [<img alt="github" src="https://img.shields.io/badge/github-rappet/rust__bgpq3-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rappet/rust_bgpq3)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/bgpq3.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bgpq3)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bgpq3-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/bgpq3)
//!
//! This library provides a thing wrapper around the [`bgpq3`]/[`bgpq4`] binary.
//!
//! See [`Bgpq3::query_v4`] and [`Bgpq3::query_v6`].
//!
//! ## Example
//!
//! bgpq3 AS-Set
//!
//! ```
//! extern crate bgpq3;
//!
//! pub fn main() {
//!     let networks = bgpq3::Bgpq3::new().query_v6("AS-RAPPET").unwrap();
//!     println!("{:?}", networks);
//! }
//! ```
//!
//! bgpq3 ASN
//!
//! ```
//! extern crate bgpq3;
//!
//! pub fn main() {
//!     let networks = bgpq3::Bgpq3::new().query_v6(207968).unwrap();
//!     println!("{:?}", networks);
//! }
//! ```
//!
//! bgpq4
//!
//! ```ignore
//! extern crate bgpq3;
//! use bgpq3::{Bgpq3, Version};
//!
//! pub fn main() {
//!     let bgpq4 = Bgpq3::builder().version(Version::Bgpq4).build();
//!     let networks = bgpq4.query_v6("AS-RAPPET").unwrap();
//!     println!("{:?}", networks);
//! }
//! ```
//!
//! ## Feature flags
//!
//! - `tokio` for async process invocation.
//!
//! [`bgpq3`]: https://github.com/snar/bgpq3
//! [`bgpq4`]: https://github.com/bgp/bgpq4

extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate ipnetwork;
#[cfg(feature = "tokio")]
extern crate tokio;

use thiserror::Error;
use ipnetwork::{Ipv4Network, Ipv6Network, IpNetworkError};
use serde::Deserialize;
use std::process::Command;
use std::fmt::{Display, Formatter};

/// A wrapper around the `bgpq3` or `bgpq4` binary.
///
/// The default wrapper can be crated with [`Bgpq3::new`].
/// A custom wrapper can be build using the [`Bgpq3Settings`] builder.
///
/// The binaries have to be located in the `$PATH` environment variable for the default configuration.
///
/// # Example
///
/// Default wrapper using `bgpq3`:
///
/// ```
/// use bgpq3::Bgpq3;
///
/// let bgpq3 = Bgpq3::new();
/// ```
///
/// Custom wrapper using the recommended bgpq4 fork:
///
/// ```
/// use bgpq3::{Bgpq3, Version};
///
/// let bgpq4 = Bgpq3::builder().version(Version::Bgpq4).build();
/// ```
#[derive(Default, Clone)]
pub struct Bgpq3 {
    settings: Bgpq3Settings,
}

impl Bgpq3 {
    /// Creates a new [`Bgpq3`] wrapper with the default settings.
    pub fn new() -> Bgpq3 {
        Default::default()
    }

    /// Creates a new wrapper with bgpq3 as a backend.
    pub fn bgpq3() -> Bgpq3 {
        Bgpq3::builder().version(Version::Bgpq3).build()
    }

    /// Creates a new wrapper with bgpq4 as a backend.
    pub fn bgpq4() -> Bgpq3 {
        Bgpq3::builder().version(Version::Bgpq4).build()
    }

    /// Creates a new [`Bgpq3`] builder [`Bgpq3Settings`].
    pub fn builder() -> Bgpq3Settings {
        Bgpq3Settings::new()
    }

    /// Creates a new [`Bgpq3`] from a builder..
    pub fn with_settings(settings: &Bgpq3Settings) -> Bgpq3 {
        Bgpq3 {
            settings: settings.clone(),
        }
    }

    /// Queries a list of IPv4 networks.
    pub fn query_v4(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv4Network>> {
        let output = query.into()
            .build_command(&self.settings, false)
            .output()?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;
        deserialized.prefixes_v4()
    }

    /// Queries a list of IPv6 networks.
    pub fn query_v6(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv6Network>> {
        let output = query.into()
            .build_command(&self.settings, true)
            .output()?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;
        deserialized.prefixes_v6()
    }

    /// Queries a list of IPv4 networks using tokio.
    #[cfg(feature = "tokio")]
    pub async fn tokio_query_v4(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv4Network>> {
        let output = tokio::process::Command::from(
            query.into()
                .build_command(&self.settings, false)
        )
            .output().await?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;
        deserialized.prefixes_v4()
    }

    /// Queries a list of IPv6 networks using tokio.
    #[cfg(feature = "tokio")]
    pub async fn tokio_query_v6(&self, query: impl Into<Bgpq3Query>) -> Bgpq3Result<Vec<Ipv6Network>> {
        let output = tokio::process::Command::from(
            query.into()
                .build_command(&self.settings, true)
        )
            .output().await?;
        if !output.status.success() {
            return Err(Bgpq3Error::StatusNotSuccess);
        }
        let deserialized: Bgpq3Output = serde_json::from_slice(output.stdout.as_slice())?;
        deserialized.prefixes_v6()
    }
}

#[derive(Deserialize)]
struct Bgpq3Output {
    #[serde(rename = "NN")]
    nn: Vec<Bgpq3OutputInner>,
}

impl Bgpq3Output {
    fn prefixes_v4(self) -> Bgpq3Result<Vec<Ipv4Network>> {
        let prefixes: Result<Vec<Ipv4Network>, IpNetworkError> = self.nn.into_iter()
            .map(|inner| inner.prefix.parse())
            .collect();
        Ok(prefixes?)
    }

    fn prefixes_v6(self) -> Bgpq3Result<Vec<Ipv6Network>> {
        let prefixes: Result<Vec<Ipv6Network>, IpNetworkError> = self.nn.into_iter()
            .map(|inner| inner.prefix.parse())
            .collect();
        Ok(prefixes?)
    }
}

#[derive(Deserialize)]
struct Bgpq3OutputInner {
    prefix: String,
}

/// A query for bgpq3.
///
/// See [`Bgpq3::query_v4`] and [`Bgpq3::query_v6`] for usage examples.
///
/// # Example
///
/// AS-SET
///
/// ```
/// use bgpq3::Bgpq3Query;
/// let query: Bgpq3Query = "AS-RAPPET".into();
/// ```
///
/// ASN
///
/// ```
/// use bgpq3::Bgpq3Query;
/// let query: Bgpq3Query = 207968.into();
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bgpq3Query {
   pub(crate) query_string: String,
}

impl Bgpq3Query {
    /// Build a query for an AS-SET
    ///
    /// # Examples
    ///
    /// ```
    /// use bgpq3::Bgpq3Query;
    /// let query = Bgpq3Query::as_set("AS-RAPPET");
    /// assert_eq!(query.to_string(), "AS-RAPPET");
    ///
    /// let query_from: Bgpq3Query = "AS-RAPPET".into();
    /// assert_eq!(query, query_from);
    /// ```
    pub fn as_set(as_set: &str) -> Bgpq3Query {
        Bgpq3Query {
            query_string: as_set.to_string(),
        }
    }

    /// Build a query for an ASN
    ///
    /// # Examples
    /// ```
    /// use bgpq3::Bgpq3Query;
    /// let query = Bgpq3Query::asn(207968);
    /// assert_eq!(query.to_string(), "AS207968");
    ///
    /// let query_from: Bgpq3Query = 207968.into();
    /// assert_eq!(query, query_from);
    /// ```
    pub fn asn(asn: u32) -> Bgpq3Query {
        Bgpq3Query {
            query_string: format!("AS{}", asn.to_string()),
        }
    }

    fn build_command(&self, settings: &Bgpq3Settings, ipv6: bool) -> Command {
        let mut command = Command::new(settings.version.bin_name());
        // JSON
        command.arg("-j");
        if ipv6 {
            command.arg("-6");
        }
        command.arg(&self.query_string);
        command
    }
}

impl Display for Bgpq3Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.query_string)
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

/// Settings to create a new [`Bgpq3`] wrapper.
///
/// [`Bgpq3`]: crate::Bgpq3
#[derive(Default, Debug, Clone)]
pub struct Bgpq3Settings {
    version: Version,
}

impl Bgpq3Settings {
    pub fn new() -> Bgpq3Settings {
        Default::default()
    }

    pub fn version(mut self, version: Version) -> Bgpq3Settings {
        self.version = version;
        self
    }

    pub fn build(self) -> Bgpq3 {
        Bgpq3::with_settings(&self)
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

/// Selects, if the bgpq3 or bgpq4 binary is used.
///
/// [`bgpq3`] is used as default.
/// [`bgpq4`] should be preferred.
///
/// [`bgpq3`]: https://github.com/snar/bgpq3
/// [`bgpq4`]: https://github.com/bgp/bgpq4
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
