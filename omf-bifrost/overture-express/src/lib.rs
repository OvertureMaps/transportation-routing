//! OvertureExpress: Fast database for Overture Maps transportation data
//!
//! This crate provides zero-copy access to transportation data using LMDB and Cap'n Proto.

#![warn(missing_docs)]

/// Placeholder for the main database interface
pub struct OvertureExpress {
    // We'll add fields as we implement
}

impl OvertureExpress {
    /// Create a new OvertureExpress instance
    pub fn new() -> Self {
        Self {
            // Empty for now
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_creation() {
        let _db = OvertureExpress::new();
        // Basic smoke test
    }
}
