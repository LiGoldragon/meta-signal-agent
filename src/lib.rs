//! Authority-verified Agent meta-policy Interface.
//!
//! Ethos owns the visible contract vocabulary. Rust exposes only encoded
//! coordinates projected through the sealed Rust Logos boundary.

pub mod bootstrap_manifest;
pub mod schema;

pub const META_SIGNAL_AGENT_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const META_SIGNAL_AGENT_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

pub use schema::lib::*;
