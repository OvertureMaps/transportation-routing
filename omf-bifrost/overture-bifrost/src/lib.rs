//! # Overture Bifrost
//! 
//! Core library for converting Overture Maps Foundation transportation data to Valhalla routing tiles.
//! 
//! This library provides the core functionality for:
//! - Reading Overture Maps GeoParquet data
//! - Converting transportation segments and connectors to Valhalla format
//! - Building administrative boundary data
//! - Managing the conversion pipeline
//! 
//! ## Modules
//! 
//! - [`cli`] - Command-line interface functionality
//! - [`core`] - Core conversion logic
//! - [`io`] - Input/output operations for various formats
//! - [`admin`] - Administrative boundary processing
//! - [`utils`] - Utility functions and helpers

pub mod cli;
pub mod core;
pub mod io;
pub mod admin;
pub mod utils;

// Re-export commonly used types
pub use overture_types;
