//! Overture Maps transportation connector types

use serde::{Deserialize, Serialize};
use geo::Point;

/// An Overture Maps transportation connector (intersection/junction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// Unique identifier for the connector
    pub id: String,
    
    /// Geometry of the connector (intersection point)
    pub geometry: Point<f64>,
    
    /// Properties associated with the connector
    pub properties: crate::properties::ConnectorProperties,
}
