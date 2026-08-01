//! Mapping functions from Overture Maps attributes to Valhalla attributes

use overture_types::AccessRestriction;

/// Valhalla road classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValhallaRoadClass {
    KMotorway,
    KTrunk,
    KPrimary,
    KSecondary,
    KTertiary,
    KResidential,
    KUnclassified,
    KServiceOther,
}

/// Valhalla access permissions with precedence tracking
#[derive(Debug, Clone)]
pub struct ValhallaAccess {
    pub k_auto_access: bool,
    pub k_bicycle_access: bool,
    pub k_bus_access: bool,
    pub k_truck_access: bool,
    pub k_pedestrian_access: bool,
    // Track what has been set and at what precedence level
    auto_set_by: Option<AccessPrecedence>,
    bicycle_set_by: Option<AccessPrecedence>,
    bus_set_by: Option<AccessPrecedence>,
    truck_set_by: Option<AccessPrecedence>,
    pedestrian_set_by: Option<AccessPrecedence>,
}

/// Access precedence levels (higher number = higher precedence)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum AccessPrecedence {
    Allowed = 1,
    Denied = 2,
    Designated = 3,
}

impl Default for ValhallaAccess {
    fn default() -> Self {
        Self {
            k_auto_access: true,
            k_bicycle_access: true,
            k_bus_access: true,
            k_truck_access: true,
            k_pedestrian_access: true,
            auto_set_by: None,
            bicycle_set_by: None,
            bus_set_by: None,
            truck_set_by: None,
            pedestrian_set_by: None,
        }
    }
}

/// Valhalla surface types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValhallaSurface {
    PavedSmooth,
    Paved,
    PavedRough,
    Compacted,
    Dirt,
    Gravel,
    Path,
    Impassable,
}

/// Maps Overture road class to Valhalla road class
pub fn map_road_class(overture_class: &str) -> ValhallaRoadClass {
    match overture_class {
        "motorway" => ValhallaRoadClass::KMotorway,
        "trunk" => ValhallaRoadClass::KTrunk,
        "primary" => ValhallaRoadClass::KPrimary,
        "secondary" => ValhallaRoadClass::KSecondary,
        "tertiary" => ValhallaRoadClass::KTertiary,
        "residential" => ValhallaRoadClass::KResidential,
        "unclassified" => ValhallaRoadClass::KUnclassified,
        "service" | "pedestrian" | "footway" | "alley" | "crosswalk" | "cycleway" | "driveway"
        | "living_street" | "parking_aisle" | "path" | "sidewalk" | "steps" | "track"
        | "unknown" => ValhallaRoadClass::KServiceOther,
        _ => ValhallaRoadClass::KServiceOther,
    }
}

/// Maps Overture access restrictions to Valhalla access permissions
pub fn map_access_restrictions(access_rules: &[AccessRestriction]) -> ValhallaAccess {
    let mut access = ValhallaAccess::default();

    // Process all rules, letting the apply_access_rule function handle precedence
    for rule in access_rules {
        apply_access_rule(&mut access, rule);
    }

    access
}

/// Helper function to apply individual access rules with precedence checking
/// Vehicle precedence is handled by order: foot > bicycle > bus > hgv > car > motor_vehicle
fn apply_access_rule(access: &mut ValhallaAccess, rule: &AccessRestriction) {
    let (access_precedence, allow) = parse_access_rule(&rule.access_type);

    // Handle vehicle precedence through order of checking (highest precedence first)
    match rule.access_type.as_str() {
        // Pedestrian access (foot) - highest vehicle precedence
        s if s.contains("foot") => {
            if should_apply_rule(access.pedestrian_set_by, access_precedence) {
                access.k_pedestrian_access = allow;
                access.pedestrian_set_by = Some(access_precedence);
            }
        }

        // Bicycle access - second highest vehicle precedence
        s if s.contains("bicycle") => {
            if should_apply_rule(access.bicycle_set_by, access_precedence) {
                access.k_bicycle_access = allow;
                access.bicycle_set_by = Some(access_precedence);
            }
        }

        // Bus access - third highest vehicle precedence
        s if s.contains("bus") => {
            if should_apply_rule(access.bus_set_by, access_precedence) {
                access.k_bus_access = allow;
                access.bus_set_by = Some(access_precedence);
            }
        }

        // Truck access (hgv) - fourth highest vehicle precedence
        s if s.contains("hgv") => {
            if should_apply_rule(access.truck_set_by, access_precedence) {
                access.k_truck_access = allow;
                access.truck_set_by = Some(access_precedence);
            }
        }

        // Auto access (car, motor_vehicle, vehicle) - lowest vehicle precedence
        s if s.contains("car") || s.contains("motor_vehicle") || s.contains("vehicle") => {
            if should_apply_rule(access.auto_set_by, access_precedence) {
                access.k_auto_access = allow;
                access.auto_set_by = Some(access_precedence);
            }
        }

        _ => {}
    }
}

/// Parse access rule to determine precedence and permission
fn parse_access_rule(access_type: &str) -> (AccessPrecedence, bool) {
    if access_type.starts_with("designated_") {
        (AccessPrecedence::Designated, true)
    } else if access_type.starts_with("denied_") {
        (AccessPrecedence::Denied, false)
    } else {
        // Default for "allowed_" and any other access type
        (AccessPrecedence::Allowed, true)
    }
}

/// Check if a rule should be applied based on precedence
fn should_apply_rule(
    current_precedence: Option<AccessPrecedence>,
    new_precedence: AccessPrecedence,
) -> bool {
    match current_precedence {
        None => true,                               // Nothing set yet, apply the rule
        Some(current) => new_precedence >= current, // Apply if new precedence is higher or equal
    }
}

/// Maps Overture surface type to Valhalla surface type
pub fn map_surface_type(surface: &str) -> ValhallaSurface {
    match surface {
        "metal" | "rubber" => ValhallaSurface::PavedSmooth,
        "paved" | "asphalt" => ValhallaSurface::Paved,
        "bricks" | "wood" => ValhallaSurface::PavedRough,
        "paving_stones" | "cobblestone" | "tiles" => ValhallaSurface::Compacted,
        "dirt" | "unpaved" => ValhallaSurface::Dirt,
        "gravel" | "shells" | "rock" => ValhallaSurface::Gravel,
        "service" => ValhallaSurface::Impassable,
        _ => ValhallaSurface::Path,
    }
}

/// Maps speed limit based on posted speed or road class defaults
pub fn map_speed_limit(speed_limit: Option<u32>, road_class: ValhallaRoadClass) -> u32 {
    // If posted speed limit is available, use it
    if let Some(speed) = speed_limit {
        return speed;
    }

    // Use defaults based on road class
    match road_class {
        ValhallaRoadClass::KMotorway => 120, // km/h
        ValhallaRoadClass::KTrunk => 100,
        ValhallaRoadClass::KPrimary => 80,
        ValhallaRoadClass::KSecondary => 60,
        ValhallaRoadClass::KTertiary => 50,
        ValhallaRoadClass::KResidential => 30,
        ValhallaRoadClass::KUnclassified => 50,
        ValhallaRoadClass::KServiceOther => 20,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_road_class() {
        assert_eq!(map_road_class("motorway"), ValhallaRoadClass::KMotorway);
        assert_eq!(map_road_class("primary"), ValhallaRoadClass::KPrimary);
        assert_eq!(map_road_class("service"), ValhallaRoadClass::KServiceOther);
        assert_eq!(map_road_class("footway"), ValhallaRoadClass::KServiceOther);
        assert_eq!(
            map_road_class("unknown_type"),
            ValhallaRoadClass::KServiceOther
        );
    }

    #[test]
    fn test_map_surface_type() {
        assert_eq!(map_surface_type("metal"), ValhallaSurface::PavedSmooth);
        assert_eq!(map_surface_type("paved"), ValhallaSurface::Paved);
        assert_eq!(map_surface_type("dirt"), ValhallaSurface::Dirt);
        assert_eq!(map_surface_type("gravel"), ValhallaSurface::Gravel);
        assert_eq!(map_surface_type("unknown"), ValhallaSurface::Path);
    }

    #[test]
    fn test_map_speed_limit() {
        // Test with posted speed limit
        assert_eq!(map_speed_limit(Some(70), ValhallaRoadClass::KPrimary), 70);

        // Test defaults
        assert_eq!(map_speed_limit(None, ValhallaRoadClass::KMotorway), 120);
        assert_eq!(map_speed_limit(None, ValhallaRoadClass::KResidential), 30);
        assert_eq!(map_speed_limit(None, ValhallaRoadClass::KServiceOther), 20);
    }

    #[test]
    fn test_map_access_restrictions_empty() {
        let access = map_access_restrictions(&[]);
        assert!(access.k_auto_access);
        assert!(access.k_bicycle_access);
        assert!(access.k_bus_access);
        assert!(access.k_truck_access);
        assert!(access.k_pedestrian_access);
    }

    #[test]
    fn test_map_access_restrictions_denied() {
        let rules = vec![AccessRestriction {
            access_type: "denied_car".to_string(),
            when: None,
        }];
        let access = map_access_restrictions(&rules);
        assert!(!access.k_auto_access);
        assert!(access.k_bicycle_access);
        assert!(access.k_bus_access);
        assert!(access.k_truck_access);
        assert!(access.k_pedestrian_access);
    }
}
