//! # Overture Types
//! 
//! Rust data types for Overture Maps transportation schema.
//! 
//! This crate provides strongly-typed Rust structures for working with
//! Overture Maps transportation data, including segments, connectors,
//! and their associated properties.

pub mod segment;
pub mod connector;
pub mod properties;

pub use segment::Segment;
pub use connector::Connector;
pub use properties::*;
