//! Overture Maps transportation segment types

use serde::{Deserialize, Serialize};
use geo::LineString;

use crate::connector::ConnectorRef;

/// An Overture Maps transportation segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Unique identifier for the segment
    pub id: String,
    
    /// Geometry of the segment (road path)
    pub geometry: LineString<f64>,
    
    /// Properties associated with the segment
    pub properties: crate::properties::SegmentProperties,

    /// References to connectors to other segments
    pub connectors: Vec<ConnectorRef>,
}
