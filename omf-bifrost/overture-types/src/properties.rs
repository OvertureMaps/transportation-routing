//! Properties for Overture Maps transportation elements

use serde::{Deserialize, Serialize};

/// Properties associated with a transportation segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentProperties {
    /// Road classification (motorway, trunk, primary, etc.)
    pub class: Option<String>,
    
    /// Subtype (road, rail, water, etc.)
    pub subtype: Option<String>,
    
    /// Surface type (paved, unpaved, gravel, etc.)
    pub surface: Option<String>,
    
    /// Road names
    pub names: Option<Names>,
    
    /// Access restrictions
    pub access_restrictions: Option<Vec<AccessRestriction>>,
    
    /// Speed limits
    pub speed_limits: Option<Vec<SpeedLimit>>,
}

/// Properties associated with a transportation connector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorProperties {
    /// Subtype (intersection, etc.)
    pub subtype: Option<String>,
    
    /// Connected segments
    pub connected_segments: Option<Vec<ConnectedSegment>>,
}

/// Road names in different languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Names {
    /// Primary name
    pub primary: Option<String>,
    
    /// Alternative names
    pub alternative: Option<Vec<String>>,
}

/// Access restriction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRestriction {
    /// Type of access (allowed, denied, etc.)
    pub access_type: String,
    
    /// When this restriction applies
    pub when: Option<AccessWhen>,
}

/// When an access restriction applies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessWhen {
    /// Vehicle access
    pub vehicle: Option<bool>,
    
    /// Bicycle access
    pub bicycle: Option<bool>,
    
    /// Pedestrian access
    pub pedestrian: Option<bool>,
}

/// Speed limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedLimit {
    /// Maximum speed
    pub max_speed: Option<Speed>,
    
    /// Minimum speed
    pub min_speed: Option<Speed>,
}

/// Speed value with unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speed {
    /// Speed value
    pub value: f64,
    
    /// Unit (mph, kmh, etc.)
    pub unit: String,
}

/// Reference to a connected segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedSegment {
    /// ID of the connected segment
    pub segment_id: String,
    
    /// Position along the segment (0.0 = start, 1.0 = end)
    pub at: f64,
}
